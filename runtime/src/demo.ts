import { ZebratronCartridgeSystem, InputManager, Button } from './index';

class Demo {
  private system: ZebratronCartridgeSystem;
  // private renderer: Renderer;
  private input: InputManager;
  private isRunning = false;
  private frameCount = 0;
  private lastTime = 0;
  private lastFrameTime = 0;
  private readonly targetFrameTime = 1000 / 60; // 16.67ms for 60 FPS
  private colorTestPressed = false;
  private soundTestPressed = false;
  private soundTestMode = false;

  constructor() {
    this.system = new ZebratronCartridgeSystem();

    // const canvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
    // this.renderer = new Renderer(canvas, 2);
    this.input = new InputManager();

    this.setupUI();
    this.setupSoundTestKeys();

    // Initialize the system (including audio)
    this.initializeSystem();
  }

  private async initializeSystem(): Promise<void> {
    try {
      console.log('🚀 Initializing ZebratronCartridgeSystem...');
      await this.system.initialize();
      console.log('✅ System initialized successfully!');
    } catch (error) {
      console.error('❌ Failed to initialize system:', error);
    }
  }

  private setupSoundTestKeys(): void {
    // Add direct keyboard event handling for number keys (sound test)
    document.addEventListener('keydown', (event) => {
      if (!this.soundTestMode) return;

      switch (event.key) {
        case '1':
          console.log('🎵 Switching to Pulse wave (0)');
          this.system.soundTestChangeWaveform(0);
          break;
        case '2':
          console.log('🎵 Switching to Sawtooth wave (1)');
          this.system.soundTestChangeWaveform(1);
          break;
        case '3':
          console.log('🎵 Switching to Triangle wave (2)');
          this.system.soundTestChangeWaveform(2);
          break;
        case '4':
          console.log('🎵 Switching to Sine wave (3)');
          this.system.soundTestChangeWaveform(3);
          break;
        case '5':
          console.log('🎵 Switching to Noise wave (4)');
          this.system.soundTestChangeWaveform(4);
          break;
      }
    });
  }

  async initialize(): Promise<void> {
    const canvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
    await this.system.initialize(canvas);

    this.updateStatus('Ready');
    console.log('ZebratronGameSystem initialized!');
  }

  private setupUI(): void {
    const startBtn = document.getElementById('startBtn') as HTMLButtonElement;
    const stopBtn = document.getElementById('stopBtn') as HTMLButtonElement;
    const resetBtn = document.getElementById('resetBtn') as HTMLButtonElement;
    const audioTestBtn = document.getElementById('audioTestBtn') as HTMLButtonElement;
    const melodyBtn = document.getElementById('melodyBtn') as HTMLButtonElement;
    const volumeSlider = document.getElementById('volumeSlider') as HTMLInputElement;
    const volumeValue = document.getElementById('volumeValue') as HTMLSpanElement;

    startBtn.addEventListener('click', () => this.start());
    stopBtn.addEventListener('click', () => this.stop());
    resetBtn.addEventListener('click', () => this.reset());
    audioTestBtn.addEventListener('click', () => this.testAudio());
    melodyBtn.addEventListener('click', () => this.toggleMelody());

    // Volume control
    volumeSlider.addEventListener('input', (e) => {
      const target = e.target as HTMLInputElement;
      const volume = parseInt(target.value) / 100;
      this.system.setMasterVolume(volume);
      volumeValue.textContent = `${target.value}%`;
    });

    this.setupFilterControls();
    this.setupDelayControls();
  }

  private setupFilterControls(): void {
    const filterEnabledCheckbox = document.getElementById('filterEnabledCheckbox') as HTMLInputElement;
    const filterTypeSelect = document.getElementById('filterTypeSelect') as HTMLSelectElement;
    const filterCutoffSlider = document.getElementById('filterCutoffSlider') as HTMLInputElement;
    const filterResonanceSlider = document.getElementById('filterResonanceSlider') as HTMLInputElement;
    const cutoffValue = document.getElementById('cutoffValue') as HTMLSpanElement;
    const resonanceValue = document.getElementById('resonanceValue') as HTMLSpanElement;

    // Filter enabled/disabled
    filterEnabledCheckbox.addEventListener('change', (e) => {
      const target = e.target as HTMLInputElement;
      this.system.setFilterEnabled(target.checked);
      console.log(`🎛️ Filter ${target.checked ? 'enabled' : 'disabled'}`);
    });

    // Filter type selection
    filterTypeSelect.addEventListener('change', (e) => {
      const target = e.target as HTMLSelectElement;
      const filterType = parseInt(target.value);
      this.system.setFilterType(filterType);
      const typeNames = ['Lowpass', 'Highpass', 'Bandpass', 'Notch'];
      console.log(`🎛️ Filter type: ${typeNames[filterType]} (${filterType})`);
    });

    // Filter cutoff frequency
    filterCutoffSlider.addEventListener('input', (e) => {
      const target = e.target as HTMLInputElement;
      const cutoff = parseInt(target.value) / 100;
      this.system.setFilterCutoff(cutoff);
      cutoffValue.textContent = `${target.value}%`;

      // Calculate approximate frequency for display
      const freq = Math.round(30 + cutoff * (20000 - 30));
      console.log(`🔧 Filter cutoff: ${cutoff.toFixed(2)} (~${freq}Hz)`);
    });

    // Filter resonance
    filterResonanceSlider.addEventListener('input', (e) => {
      const target = e.target as HTMLInputElement;
      const resonance = parseInt(target.value) / 100;
      this.system.setFilterResonance(resonance);
      resonanceValue.textContent = `${target.value}%`;

      if (resonance > 0.7) {
        console.log(`🌊 High resonance: ${resonance.toFixed(2)} - entering self-oscillation!`);
      } else {
        console.log(`🌊 Filter resonance: ${resonance.toFixed(2)}`);
      }
    });
  }

  private setupDelayControls(): void {
    const delayEnabledCheckbox = document.getElementById('delayEnabledCheckbox') as HTMLInputElement;
    const delayTimeSlider = document.getElementById('delayTimeSlider') as HTMLInputElement;
    const delayFeedbackSlider = document.getElementById('delayFeedbackSlider') as HTMLInputElement;
    const delayMixSlider = document.getElementById('delayMixSlider') as HTMLInputElement;
    const delayTimeValue = document.getElementById('delayTimeValue') as HTMLSpanElement;
    const delayFeedbackValue = document.getElementById('delayFeedbackValue') as HTMLSpanElement;
    const delayMixValue = document.getElementById('delayMixValue') as HTMLSpanElement;

    // Delay enabled/disabled
    delayEnabledCheckbox.addEventListener('change', (e) => {
      const target = e.target as HTMLInputElement;
      this.system.setDelayEnabled(target.checked);
      console.log(`🔊 Delay ${target.checked ? 'enabled' : 'disabled'}`);
    });

    // Delay time (0-1000ms)
    delayTimeSlider.addEventListener('input', (e) => {
      const target = e.target as HTMLInputElement;
      const delayTime = parseInt(target.value) / 100; // 0.0 to 1.0
      this.system.setDelayTime(delayTime);

      // Convert to milliseconds for display
      const delayMs = Math.round(delayTime * 1000);
      delayTimeValue.textContent = `${delayMs}ms`;

      console.log(`⏱️ Delay time: ${delayMs}ms`);
    });

    // Delay feedback
    delayFeedbackSlider.addEventListener('input', (e) => {
      const target = e.target as HTMLInputElement;
      const feedback = parseInt(target.value) / 100;
      this.system.setDelayFeedback(feedback);
      delayFeedbackValue.textContent = `${target.value}%`;

      if (feedback > 0.8) {
        console.log(`🔄 High feedback: ${feedback.toFixed(2)} - entering infinite echo territory!`);
      } else {
        console.log(`🔄 Delay feedback: ${feedback.toFixed(2)}`);
      }
    });

    // Delay mix (dry/wet balance)
    delayMixSlider.addEventListener('input', (e) => {
      const target = e.target as HTMLInputElement;
      const mix = parseInt(target.value) / 100;
      this.system.setDelayMix(mix);
      delayMixValue.textContent = `${target.value}%`;

      let description = '';
      if (mix < 0.25) description = ' (mostly dry)';
      else if (mix > 0.75) description = ' (mostly wet)';
      else if (Math.abs(mix - 0.5) < 0.1) description = ' (balanced)';

      console.log(`🎚️ Delay mix: ${mix.toFixed(2)}${description}`);
    });
  }

  private async testAudio(): Promise<void> {
    console.log('🎵 === AUDIO TEST STARTED ===');

    try {
      console.log('🔊 Audio available:', this.system.isAudioAvailable());

      console.log('🔊 Audio available:', this.system.isAudioAvailable());
      console.log('🎛️ Audio info:', this.system.getAudioInfo());

      // Force enter sound test mode
      console.log('🎼 Entering sound test mode...');
      this.system.enterSoundTestMode();
      this.soundTestMode = true;

      console.log('✅ Sound test active:', this.system.isSoundTestMode());
      console.log('🎵 Waveform:', this.system.getCurrentWaveform());
      console.log('🎵 Note:', this.system.getCurrentNote());

      // Try changing to pulse wave and middle C
      console.log('🎛️ Setting pulse wave (waveform 0)');
      this.system.soundTestChangeWaveform(0);

      console.log('🎼 Setting to middle C (note 60)');
      this.system.soundTestChangeNote(60);

      console.log('🔊 Final state - Waveform:', this.system.getCurrentWaveform(), 'Note:', this.system.getCurrentNote());

      // Check if system is actually running
      if (!this.system.isRunning()) {
        console.log('⚠️ System not running, starting...');
        await this.system.start();
      }

      // Manual test - try to get samples directly from the APU
      try {
        console.log('🧪 Testing direct sample generation...');
        const debugSamples = this.system.generateDebugSamples(5);
        console.log('🎵 Direct APU samples:', debugSamples);

        if (debugSamples.every(sample => sample === 0)) {
          console.log('⚠️ All samples are zero - APU might not be generating audio');
        } else {
          console.log('✅ APU is generating non-zero samples!');
        }
      } catch (error) {
        console.error('❌ Direct sample test failed:', error);
      }

      console.log('🎵 === AUDIO TEST COMPLETE ===');
    } catch (error) {
      console.error('❌ Audio test failed:', error);
    }
  }

  private async start(): Promise<void> {
    if (this.isRunning) return;

    try {
      console.log('🎮 Starting game loop...');
      console.log('🔊 Audio available:', this.system.isAudioAvailable());
      console.log('🏃 System running:', this.system.isRunning());
      console.log('🎵 Audio system status:', this.system.isAudioAvailable());
      console.log('🎛️ Audio info:', this.system.getAudioInfo());

      // Ensure sound test mode is off for normal game operation
      console.log('🧪 Ensuring normal game audio mode...');
      this.system.exitSoundTestMode();
      this.system.setMelodyEnabled(false); // Disable background melody

      setTimeout(() => {
        console.log('🎼 Sound test active:', this.system.isSoundTestMode());
        console.log('🎵 Melody enabled:', this.system.getMelodyEnabled());
        console.log('🎵 System ready for game sounds');
      }, 100);

      // Actually start the WASM system (this will auto-load platformer cartridge)
      console.log('🚀 Starting WASM system...');
      await this.system.start();
      console.log('✅ WASM system started, running:', this.system.isRunning());

    } catch (error) {
      console.error('❌ Failed to start system:', error);
    }

    this.isRunning = true;

    this.updateStatus('Running');
    this.gameLoop();
  }

  private stop(): void {
    this.isRunning = false;
    this.system.stop();
    this.updateStatus('Stopped');
  }

  private reset(): void {
    this.system.reset();
    this.updateStatus('Reset');
  }

  private gameLoop = (currentTime: number = 0): void => {
    if (!this.isRunning) return;

    // Calculate time since last game update
    const deltaTime = currentTime - this.lastFrameTime;

    // Only update game logic at 60 FPS, regardless of display refresh rate
    if (deltaTime >= this.targetFrameTime) {
      this.lastFrameTime = currentTime - (deltaTime % this.targetFrameTime);

      // Calculate FPS (for display purposes)
      if (currentTime - this.lastTime >= 1000) {
        const fps = this.frameCount;
        this.frameCount = 0;
        this.lastTime = currentTime;
        this.updateFPS(fps);
      }

      // Handle input for sprite movement
      const up = this.input.isPressed(Button.Up);
      const down = this.input.isPressed(Button.Down);
      const left = this.input.isPressed(Button.Left);
      const right = this.input.isPressed(Button.Right);

      // Toggle color test mode with Enter key
      if (this.input.isPressed(Button.Start)) { // Using Start button for demo
        // Only toggle once per press (simple debouncing)
        if (!this.colorTestPressed) {
          this.system.toggleColorTest();
          this.colorTestPressed = true;
        }
      } else {
        this.colorTestPressed = false;
      }

      // Toggle sound test mode with 'S' key
      if (this.input.isPressed(Button.Select)) { // Using Select button for sound test
        if (!this.soundTestPressed) {
          this.soundTestMode = !this.soundTestMode;
          if (this.soundTestMode) {
            this.system.enterSoundTestMode();
          } else {
            this.system.exitSoundTestMode();
          }
          this.soundTestPressed = true;
        }
      } else {
        this.soundTestPressed = false;
      }

      // Sound test controls when in sound test mode
      if (this.soundTestMode) {
        this.handleSoundTestControls();
      }

      this.system.handleInput(up, down, left, right);

      // Step the system for one frame
      const frameReady = this.system.stepFrame();
      if (frameReady) {
        this.system.render();
        this.frameCount++;

        // Debug logging every 60 frames (once per second)
        if (this.frameCount % 60 === 0) {
          console.log(`🎮 Frame ${this.frameCount}, System running: ${this.system.isRunning()}`);
        }
      } else {
        // Debug why frames aren't ready
        if (this.frameCount < 5) {
          console.log(`⚠️ Frame not ready, system running: ${this.system.isRunning()}`);
        }
      }

      // Update debug info
      this.updateDebugInfo();
    }

    // Continue the loop at display refresh rate
    requestAnimationFrame(this.gameLoop);
  };

  private updateStatus(status: string): void {
    const statusElement = document.getElementById('status');
    if (statusElement) {
      statusElement.textContent = status;
    }
  }

  private updateFPS(fps: number): void {
    const fpsElement = document.getElementById('frameRate');
    if (fpsElement) {
      fpsElement.textContent = `FPS: ${fps}`;
    }
  }

  private handleSoundTestControls(): void {
    // Number keys 1-5 handled by direct keyboard events in setupSoundTestKeys()
    // This method now only handles arrow key navigation

    // Arrow keys for note control
    if (this.input.isPressed(Button.Up)) {
      const currentNote = this.system.getCurrentNote();
      this.system.soundTestChangeNote(Math.min(108, currentNote + 1));
    }
    if (this.input.isPressed(Button.Down)) {
      const currentNote = this.system.getCurrentNote();
      this.system.soundTestChangeNote(Math.max(21, currentNote - 1));
    }

    // Left/Right for octaves
    if (this.input.isPressed(Button.Left)) {
      const currentNote = this.system.getCurrentNote();
      this.system.soundTestChangeNote(Math.max(21, currentNote - 12));
    }
    if (this.input.isPressed(Button.Right)) {
      const currentNote = this.system.getCurrentNote();
      this.system.soundTestChangeNote(Math.min(108, currentNote + 12));
    }
  }

  private updateDebugInfo(): void {
    const cpuStateElement = document.getElementById('cpuState');
    if (cpuStateElement) {
      if (this.soundTestMode) {
        // Show sound test info
        const waveformNames = ['Pulse', 'Sawtooth', 'Triangle', 'Sine', 'Noise'];
        const currentWaveform = this.system.getCurrentWaveform();
        const currentNote = this.system.getCurrentNote();
        const noteName = this.midiNoteToName(currentNote);

        const audioInfo = this.system.getAudioInfo();
        const audioStatus = this.system.isAudioAvailable() ? '🔊 Active' : '🔇 Unavailable';

        cpuStateElement.innerHTML = `
          <strong>🎵 SOUND TEST MODE 🎵</strong><br>
          Audio: ${audioStatus}<br>
          Waveform: ${waveformNames[currentWaveform]} (${currentWaveform + 1})<br>
          Note: ${noteName} (MIDI ${currentNote})<br>
          Frequency: ${this.midiToFrequency(currentNote).toFixed(1)} Hz<br>
          ${audioInfo ? `Sample Rate: ${audioInfo.sampleRate} Hz<br>` : ''}
          ${audioInfo ? `Latency: ~${audioInfo.estimatedLatency}ms<br>` : ''}
          <br>
          <small>Controls:</small><br>
          <small>1-5: Change waveform</small><br>
          <small>↑↓: Semitone up/down</small><br>
          <small>←→: Octave down/up</small><br>
          <small>Tab: Exit sound test</small>
        `;
      } else {
        const state = this.system.getCpuState();
        if (state && typeof state === 'object') {
          cpuStateElement.innerHTML = `
            CPU: PC=$${(state.pc || 0).toString(16).padStart(4, '0')}
            A=$${(state.a || 0).toString(16).padStart(2, '0')}
            X=$${(state.x || 0).toString(16).padStart(2, '0')}
            Y=$${(state.y || 0).toString(16).padStart(2, '0')}
            <br>
            SP=$${(state.sp || 0).toString(16).padStart(2, '0')}
            Status=$${(state.status || 0).toString(16).padStart(2, '0')}
            Cycles: ${state.cycles || 0}
          `;
        } else {
          cpuStateElement.innerHTML = 'CPU: Not initialized or invalid state';
        }
      }
    }
  }

  private midiNoteToName(midiNote: number): string {
    const noteNames = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
    const octave = Math.floor(midiNote / 12) - 1;
    const noteName = noteNames[midiNote % 12];
    return `${noteName}${octave}`;
  }

  private midiToFrequency(midiNote: number): number {
    return 440 * Math.pow(2, (midiNote - 69) / 12);
  }

  private toggleMelody(): void {
    console.log('🎼 Toggling Russian melody demo...');

    const melodyBtn = document.getElementById('melodyBtn') as HTMLButtonElement;
    if (!melodyBtn) {
      console.error('❌ Melody button not found');
      return;
    }

    try {
      const isCurrentlyEnabled = this.system.getMelodyEnabled();
      console.log('🎵 Current melody state:', isCurrentlyEnabled);

      if (isCurrentlyEnabled) {
        // Turn off melody
        this.system.setMelodyEnabled(false);
        melodyBtn.textContent = '🎼 Play Melody';
        melodyBtn.style.background = '#660066';
        console.log('🔇 Russian melody disabled');
      } else {
        // Turn on melody
        this.system.setMelodyEnabled(true);
        melodyBtn.textContent = '🔇 Stop Melody';
        melodyBtn.style.background = '#006600';
        console.log('🎵 Russian melody enabled - haunting D minor melody playing...');
      }
    } catch (error) {
      console.error('❌ Error toggling melody:', error);
    }
  }
}

// Initialize the demo when the page loads
const demo = new Demo();
demo.initialize().then(() => {
  console.log('Demo ready!');
}).catch((error) => {
  console.error('Failed to initialize demo:', error);
});