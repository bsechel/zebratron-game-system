/**
 * Web Audio API integration for ZebratronGameSystem
 * Connects the Rust APU's digital oscillators to browser audio output
 */

export class AudioManager {
  private audioContext: AudioContext | null = null;
  private gainNode: GainNode | null = null;
  private scriptProcessor: ScriptProcessorNode | null = null;
  private isInitialized = false;
  private isPlaying = false;

  constructor() {}

  /**
   * Initialize the Web Audio API
   * Must be called after user interaction due to browser autoplay policies
   */
  async initialize(): Promise<void> {
    if (this.isInitialized) return;

    try {
      // Create audio context
      this.audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();

      // Resume context if suspended (required by some browsers)
      if (this.audioContext.state === 'suspended') {
        await this.audioContext.resume();
      }

      // Create gain node for volume control
      this.gainNode = this.audioContext.createGain();
      this.gainNode.gain.setValueAtTime(0.3, this.audioContext.currentTime); // Start at 30% volume
      this.gainNode.connect(this.audioContext.destination);

      // Create script processor for real-time audio generation
      // Using 1024 buffer size for lower latency (~23ms at 44.1kHz)
      this.scriptProcessor = this.audioContext.createScriptProcessor(1024, 0, 1);
      this.scriptProcessor.connect(this.gainNode);

      this.isInitialized = true;
      console.log('🎵 Audio system initialized successfully');
      console.log(`Sample rate: ${this.audioContext.sampleRate} Hz`);
      console.log(`Buffer size: 1024 samples`);
      console.log(`Estimated latency: ~${Math.round(1024 / this.audioContext.sampleRate * 1000)}ms`);
    } catch (error) {
      console.error('Failed to initialize audio:', error);
      throw error;
    }
  }

  /**
   * Connect the ZebratronSystem's APU to audio output
   */
  connectSystem(system: any): void {
    if (!this.isInitialized || !this.scriptProcessor) {
      throw new Error('Audio not initialized. Call initialize() first.');
    }

    console.log('🔧 Connecting APU to Web Audio API...');
    console.log('Testing audio sample generation...');

    // Test if we can generate samples
    try {
      const testSample = system.generate_audio_sample();
      console.log('✅ Generated test sample:', testSample);
    } catch (error) {
      console.error('❌ Failed to generate test sample:', error);
      console.log('🔧 Falling back to test tone generator...');
      this.connectTestTone();
      return;
    }

    let debugSampleCount = 0;

    // Set up the audio processing callback
    this.scriptProcessor.onaudioprocess = (event) => {
      const outputBuffer = event.outputBuffer;
      const outputData = outputBuffer.getChannelData(0); // Mono output

      // Fill buffer with samples from the APU
      for (let i = 0; i < outputBuffer.length; i++) {
        try {
          // Get sample from the Rust APU
          const sample = system.generate_audio_sample();
          outputData[i] = sample;

          // Debug: Log first few samples to check if we're getting data
          if (debugSampleCount < 10) {
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

    console.log('🎛️ APU connected to Web Audio API');
  }

  /**
   * Fallback test tone generator to verify Web Audio is working
   */
  private connectTestTone(): void {
    console.log('🎵 Using test tone generator (440Hz sine wave)');

    let phase = 0;
    const frequency = 440; // A4 note

    this.scriptProcessor!.onaudioprocess = (event) => {
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

    console.log('🔊 Test tone connected - you should hear a 440Hz sine wave');
  }

  /**
   * Start audio playback
   * Must be called after user interaction
   */
  async start(): Promise<void> {
    if (!this.isInitialized) {
      throw new Error('Audio not initialized');
    }

    if (this.isPlaying) return;

    // Resume audio context if needed
    if (this.audioContext!.state === 'suspended') {
      await this.audioContext!.resume();
    }

    this.isPlaying = true;
    console.log('🔊 Audio playback started');
  }

  /**
   * Stop audio playback
   */
  stop(): void {
    this.isPlaying = false;
    console.log('🔇 Audio playback stopped');
  }

  /**
   * Set master volume (0.0 to 1.0)
   */
  setVolume(volume: number): void {
    if (!this.gainNode) return;

    // Clamp volume to safe range
    const clampedVolume = Math.max(0, Math.min(1, volume));

    // Use exponential ramp for smoother volume changes
    const now = this.audioContext!.currentTime;
    this.gainNode.gain.exponentialRampToValueAtTime(
      clampedVolume || 0.001, // Avoid zero for exponential ramp
      now + 0.1
    );
  }

  /**
   * Get current volume
   */
  getVolume(): number {
    return this.gainNode ? this.gainNode.gain.value : 0;
  }

  /**
   * Check if audio is available and initialized
   */
  isAvailable(): boolean {
    return this.isInitialized && this.audioContext !== null;
  }

  /**
   * Get audio context info for debugging
   */
  getAudioInfo(): any {
    if (!this.audioContext) return null;

    return {
      sampleRate: this.audioContext.sampleRate,
      state: this.audioContext.state,
      currentTime: this.audioContext.currentTime,
      bufferSize: 4096,
      estimatedLatency: Math.round(4096 / this.audioContext.sampleRate * 1000)
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
    console.log('🔌 Audio system disposed');
  }
}