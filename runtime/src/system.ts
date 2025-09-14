import init, { ZebratronSystem as WasmSystem } from '../pkg/zebratron_core.js';

export class ZebratronSystem {
  private wasmSystem: WasmSystem | null = null;
  private canvas: HTMLCanvasElement | null = null;
  private ctx: CanvasRenderingContext2D | null = null;
  // private audioCtx: AudioContext | null = null;
  private isInitialized = false;

  async initialize(canvasElement?: HTMLCanvasElement): Promise<void> {
    if (this.isInitialized) return;

    // Initialize the WebAssembly module
    await init();
    this.wasmSystem = new WasmSystem();

    // Set up canvas if provided
    if (canvasElement) {
      this.setupCanvas(canvasElement);
    }

    // Set up audio context
    // this.setupAudio();

    this.isInitialized = true;
  }

  private setupCanvas(canvas: HTMLCanvasElement): void {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d');

    if (!this.ctx) {
      throw new Error('Failed to get 2D rendering context');
    }

    // Set canvas size to match system resolution
    canvas.width = 320;
    canvas.height = 240;
  }

  // private setupAudio(): void {
  //   try {
  //     this.audioCtx = new (window.AudioContext || (window as any).webkitAudioContext)();
  //   } catch (e) {
  //     console.warn('Audio context not supported');
  //   }
  // }

  loadCartridge(romData: Uint8Array): boolean {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.load_cartridge(romData);
  }

  start(): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.start();
  }

  stop(): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.stop();
  }

  reset(): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.reset();
  }

  stepFrame(): boolean {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.step_frame();
  }

  render(): void {
    if (!this.ctx || !this.canvas || !this.wasmSystem) {
      return;
    }

    const width = this.wasmSystem.get_screen_width();
    const height = this.wasmSystem.get_screen_height();
    const buffer = this.wasmSystem.get_screen_buffer();

    // Create ImageData from the WASM buffer
    const imageData = new ImageData(
      new Uint8ClampedArray(buffer),
      width,
      height
    );

    // Draw to canvas
    this.ctx.putImageData(imageData, 0, 0);
  }

  isRunning(): boolean {
    if (!this.wasmSystem) {
      return false;
    }
    return this.wasmSystem.is_running();
  }

  getCpuState(): any {
    if (!this.wasmSystem) {
      return null;
    }
    try {
      const state = this.wasmSystem.get_cpu_state();
      console.log('CPU state from WASM:', state);
      return state;
    } catch (error) {
      console.error('Error getting CPU state:', error);
      return null;
    }
  }

  readMemory(address: number): number {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.read_memory(address);
  }

  writeMemory(address: number, value: number): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.write_memory(address, value);
  }

  handleInput(up: boolean, down: boolean, left: boolean, right: boolean): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    // Forward input to the WASM system which will handle it internally
    this.wasmSystem.handle_input(up, down, left, right);
  }

  toggleColorTest(): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.toggle_color_test();
  }

  // Sound test methods
  enterSoundTestMode(): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.enter_sound_test_mode();
  }

  exitSoundTestMode(): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.exit_sound_test_mode();
  }

  soundTestChangeWaveform(waveform: number): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.sound_test_change_waveform(waveform);
  }

  soundTestChangeNote(note: number): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.sound_test_change_note(note);
  }

  soundTestSetPulseWidth(width: number): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.sound_test_set_pulse_width(width);
  }

  getCurrentWaveform(): number {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.get_current_waveform();
  }

  getCurrentNote(): number {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.get_current_note();
  }

  isSoundTestMode(): boolean {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.is_sound_test_mode();
  }
}