/**
 * Web Audio API integration for ZebratronGameSystem
 * Connects the Rust APU's digital oscillators to browser audio output
 */

// Flip to true to re-enable verbose audio pipeline logging (Chrome autoplay/AudioContext debugging).
// Kept as a flag rather than deleting/commenting the logging, since this may need debugging again
// on other browsers or the native Pi audio path.
const AUDIO_DEBUG = false;

export class AudioManager {
  private audioContext: AudioContext | null = null;
  private gainNode: GainNode | null = null;
  private scriptProcessor: ScriptProcessorNode | null = null;
  private isInitialized = false;
  private isPlaying = false;
  private connectedSystem: any = null; // Store system reference for callback setup
  private useTestTone = false;

  constructor() {}

  /**
   * Initialize the Web Audio API
   * Defer actual AudioContext creation until user interaction (start())
   */
  async initialize(): Promise<void> {
    if (this.isInitialized) return;
    this.isInitialized = true;
    if (AUDIO_DEBUG) console.log('🎵 AudioManager initialized (AudioContext creation deferred until user interaction)');
  }

  /**
   * Connect the ZebratronSystem's APU to audio output
   * Stores the system reference but defers callback setup until start()
   */
  connectSystem(system: any): void {
    if (!this.isInitialized) {
      throw new Error('Audio not initialized. Call initialize() first.');
    }

    if (AUDIO_DEBUG) console.log('🔧 Storing system reference for audio connection...');

    // Test if we can generate samples
    try {
      const testSample = system.generate_audio_sample();
      if (AUDIO_DEBUG) console.log('✅ Generated test sample:', testSample);
      this.useTestTone = false;
    } catch (error) {
      console.error('❌ Failed to generate test sample:', error);
      if (AUDIO_DEBUG) console.log('🔧 Will fall back to test tone generator on start...');
      this.useTestTone = true;
    }

    // Store system reference - callback will be set up in start() after context is running
    this.connectedSystem = system;
    if (AUDIO_DEBUG) console.log('✅ System stored - callback will be activated when audio starts');
  }

  /**
   * Set up the actual audio callback (called from start() after context is running)
   */
  private setupAudioCallback(): void {
    if (!this.connectedSystem || !this.scriptProcessor) {
      console.warn('⚠️ Cannot setup callback - no system connected');
      return;
    }

    const system = this.connectedSystem;
    let debugSampleCount = 0;
    let callbackCount = 0;
    let nonZeroSamples = 0;

    if (AUDIO_DEBUG) console.log('🔧 Setting up audio callback NOW (context is running)...');

    // Set up the audio processing callback
    this.scriptProcessor.onaudioprocess = (event) => {
      callbackCount++;

      if (AUDIO_DEBUG) {
        // Log first few callbacks to verify they're firing
        if (callbackCount <= 5) {
          console.log(`🎵 Audio callback #${callbackCount} - CALLBACK IS FIRING!`);
        }

        // Log every 60 callbacks (roughly once per second at 44.1kHz with 1024 buffer)
        if (callbackCount % 60 === 0) {
          console.log(`🎵 Audio callback #${callbackCount} - Generated ${nonZeroSamples} non-zero samples so far`);
        }
      }

      const outputBuffer = event.outputBuffer;
      const outputData = outputBuffer.getChannelData(0); // Mono output

      // Fill buffer with samples from the APU
      for (let i = 0; i < outputBuffer.length; i++) {
        try {
          // Get sample from the Rust APU
          const sample = system.generate_audio_sample();
          outputData[i] = sample;

          // Track non-zero samples
          if (sample !== 0) {
            nonZeroSamples++;
          }

          // Debug: Log first few samples to check if we're getting data
          if (AUDIO_DEBUG && debugSampleCount < 10) {
            console.log(`APU Sample ${debugSampleCount}:`, sample);
            debugSampleCount++;
          }
        } catch (error) {
          console.error('Error generating sample:', error);
          // If APU fails, output silence to prevent audio artifacts
          outputData[i] = 0;
        }
      }
    };

    if (AUDIO_DEBUG) console.log('✅ Audio callback attached and ready!');
  }

  /**
   * Fallback test tone generator to verify Web Audio is working
   */
  private connectTestTone(): void {
    if (!this.scriptProcessor) return;

    if (AUDIO_DEBUG) console.log('🎵 Using test tone generator (440Hz sine wave)');

    let phase = 0;
    const frequency = 440; // A4 note

    this.scriptProcessor.onaudioprocess = (event) => {
      const outputBuffer = event.outputBuffer;
      const outputData = outputBuffer.getChannelData(0);
      const sampleRate = this.audioContext!.sampleRate;

      for (let i = 0; i < outputBuffer.length; i++) {
        // Generate simple sine wave
        outputData[i] = Math.sin(phase) * 0.1; // Quiet volume
        phase += (2 * Math.PI * frequency) / sampleRate;

        // Keep phase in reasonable bounds
        if (phase > 2 * Math.PI) {
          phase -= 2 * Math.PI;
        }
      }
    };

    if (AUDIO_DEBUG) console.log('🔊 Test tone connected - you should hear a 440Hz sine wave');
  }

  /**
   * Start audio playback
   * Must be called after user interaction
   */
  async start(): Promise<void> {
    if (!this.isInitialized) {
      throw new Error('Audio not initialized');
    }

    // Create the AudioContext lazily here under user gesture
    if (!this.audioContext) {
      if (AUDIO_DEBUG) console.log('🎵 Creating AudioContext under user gesture...');
      try {
        this.audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
        if (AUDIO_DEBUG) console.log('✅ AudioContext created, sample rate:', this.audioContext.sampleRate);

        // Create gain node for volume control
        this.gainNode = this.audioContext.createGain();
        this.gainNode.gain.setValueAtTime(0.3, this.audioContext.currentTime); // Start at 30% volume
        this.gainNode.connect(this.audioContext.destination);

        // Create script processor for real-time audio generation
        this.scriptProcessor = this.audioContext.createScriptProcessor(1024, 0, 1);
        this.scriptProcessor.connect(this.gainNode);

        if (AUDIO_DEBUG) console.log('✅ Audio graph constructed');
      } catch (error) {
        console.error('❌ Failed to construct AudioContext:', error);
        throw error;
      }
    }

    if (AUDIO_DEBUG) console.log('🎵 Starting audio - current state:', this.audioContext.state);

    // CRITICAL: Always call resume() even if the state is already 'running'.
    // Chrome sometimes restores the AudioContext in 'running' state on reload,
    // but silent-blocks actual hardware output until resume() is explicitly called within a user gesture.
    if (AUDIO_DEBUG) console.log('🔓 Resuming audio context to ensure Chrome unblocks output...');
    await this.audioContext.resume();
    if (AUDIO_DEBUG) console.log('✅ Audio context resume called, state:', this.audioContext.state);

    // Force resume again if still suspended (some browsers need multiple attempts)
    if (this.audioContext.state === 'suspended') {
      if (AUDIO_DEBUG) console.log('⚠️ Still suspended, trying again...');
      await this.audioContext.resume();
      if (AUDIO_DEBUG) console.log('🔄 Second resume attempt, state:', this.audioContext.state);
    }

    // Check if we successfully resumed
    if (this.audioContext.state !== 'running') {
      console.error('❌ Failed to start audio context, state:', this.audioContext.state);
      throw new Error(`Audio context is ${this.audioContext.state}, expected running`);
    }

    if (this.isPlaying) {
      if (AUDIO_DEBUG) console.log('🔊 Audio already playing');
      return;
    }

    // CRITICAL: Set up the audio callback NOW that context is running
    if (this.useTestTone) {
      this.connectTestTone();
    } else if (this.connectedSystem) {
      if (AUDIO_DEBUG) console.log('🔧 Context is running - setting up audio callback...');
      this.setupAudioCallback();
    } else {
      console.warn('⚠️ No system connected - cannot set up callback');
    }

    this.isPlaying = true;
    if (AUDIO_DEBUG) {
      console.log('🔊 Audio playback started successfully, context state:', this.audioContext.state);
      console.log('🔊 ScriptProcessor connected:', !!this.scriptProcessor);
      console.log('🔊 GainNode connected:', !!this.gainNode);
    }
  }

  /**
   * Stop audio playback
   */
  stop(): void {
    this.isPlaying = false;
    if (AUDIO_DEBUG) console.log('🔇 Audio playback stopped');
    if (this.audioContext && this.audioContext.state === 'running') {
      this.audioContext.suspend().then(() => {
        if (AUDIO_DEBUG) console.log('⏸️ AudioContext suspended');
      }).catch(err => {
        console.warn('⚠️ Failed to suspend AudioContext:', err);
      });
    }
  }

  /**
   * Set master volume (0.0 to 1.0)
   */
  setVolume(volume: number): void {
    if (!this.gainNode || !this.audioContext) return;

    // Clamp volume to safe range
    const clampedVolume = Math.max(0, Math.min(1, volume));

    // Use exponential ramp for smoother volume changes
    const now = this.audioContext.currentTime;
    this.gainNode.gain.exponentialRampToValueAtTime(
      clampedVolume || 0.001, // Avoid zero for exponential ramp
      now + 0.1
    );
  }

  /**
   * Get current volume
   */
  getVolume(): number {
    return this.gainNode ? this.gainNode.gain.value : 0.3;
  }

  /**
   * Check if audio is available and initialized
   */
  isAvailable(): boolean {
    return this.isInitialized;
  }

  /**
   * Get audio context info for debugging
   */
  getAudioInfo(): any {
    if (!this.audioContext) {
      return {
        sampleRate: 44100,
        state: 'suspended',
        currentTime: 0,
        bufferSize: 1024,
        estimatedLatency: 23
      };
    }

    return {
      sampleRate: this.audioContext.sampleRate,
      state: this.audioContext.state,
      currentTime: this.audioContext.currentTime,
      bufferSize: 1024,
      estimatedLatency: Math.round(1024 / this.audioContext.sampleRate * 1000)
    };
  }

  /**
   * Cleanup audio resources
   */
  dispose(): void {
    if (this.scriptProcessor) {
      this.scriptProcessor.disconnect();
      this.scriptProcessor = null;
    }

    if (this.gainNode) {
      this.gainNode.disconnect();
      this.gainNode = null;
    }

    if (this.audioContext && this.audioContext.state !== 'closed') {
      this.audioContext.close();
      this.audioContext = null;
    }

    this.isInitialized = false;
    this.isPlaying = false;
    if (AUDIO_DEBUG) console.log('🔌 Audio system disposed');
  }
}
