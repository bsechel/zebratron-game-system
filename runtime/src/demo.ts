import { ZebratronSystem, InputManager, Button } from './index.js';

class Demo {
  private system: ZebratronSystem;
  // private renderer: Renderer;
  private input: InputManager;
  private isRunning = false;
  private frameCount = 0;
  private lastTime = 0;
  private colorTestPressed = false;
  private soundTestPressed = false;
  private soundTestMode = false;

  constructor() {
    this.system = new ZebratronSystem();

    // const canvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
    // this.renderer = new Renderer(canvas, 2);
    this.input = new InputManager();

    this.setupUI();
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

    startBtn.addEventListener('click', () => this.start());
    stopBtn.addEventListener('click', () => this.stop());
    resetBtn.addEventListener('click', () => this.reset());
  }

  private start(): void {
    if (this.isRunning) return;

    // Create a simple test ROM (just fills memory with test pattern)
    const testRom = new Uint8Array(1024);
    for (let i = 0; i < testRom.length; i++) {
      testRom[i] = i % 256;
    }

    this.system.loadCartridge(testRom);
    this.system.start();
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

    // Calculate FPS
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
    if (this.system.stepFrame()) {
      this.system.render();
      this.frameCount++;
    }

    // Update debug info
    this.updateDebugInfo();

    // Continue the loop
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
    // Number keys 1-5 for waveform selection
    if (this.input.isPressed('Digit1' as any)) this.system.soundTestChangeWaveform(0); // Pulse
    if (this.input.isPressed('Digit2' as any)) this.system.soundTestChangeWaveform(1); // Saw
    if (this.input.isPressed('Digit3' as any)) this.system.soundTestChangeWaveform(2); // Triangle
    if (this.input.isPressed('Digit4' as any)) this.system.soundTestChangeWaveform(3); // Sine
    if (this.input.isPressed('Digit5' as any)) this.system.soundTestChangeWaveform(4); // Noise

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

        cpuStateElement.innerHTML = `
          <strong>🎵 SOUND TEST MODE 🎵</strong><br>
          Waveform: ${waveformNames[currentWaveform]} (${currentWaveform + 1})<br>
          Note: ${noteName} (MIDI ${currentNote})<br>
          Frequency: ${this.midiToFrequency(currentNote).toFixed(1)} Hz<br>
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
}

// Initialize the demo when the page loads
const demo = new Demo();
demo.initialize().then(() => {
  console.log('Demo ready!');
}).catch((error) => {
  console.error('Failed to initialize demo:', error);
});