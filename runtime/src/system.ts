import init, { ZebratronSystem as WasmSystem, ZebratronCartridgeSystem as WasmCartridgeSystem } from '../pkg/zebratron_core.js';
import { AudioManager } from './audio';
import { MidiManager } from './midi';

export class ZebratronSystem {
  private wasmSystem: WasmSystem | null = null;
  private canvas: HTMLCanvasElement | null = null;
  private ctx: CanvasRenderingContext2D | null = null;
  private audioManager: AudioManager | null = null;
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

    // Set up audio system
    console.log('🎧 Initializing AudioManager...');
    this.audioManager = new AudioManager();
    await this.audioManager.initialize();

    console.log('🔗 Connecting WASM system to audio...');
    console.log('WASM system available:', !!this.wasmSystem);
    this.audioManager.connectSystem(this.wasmSystem);

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

  async start(): Promise<void> {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }

    this.wasmSystem.start();

    // Start audio playback (requires user interaction)
    if (this.audioManager) {
      try {
        console.log('🎵 Starting audio playback...');
        await this.audioManager.start();

        // Make sure the APU is connected to audio output
        console.log('🔗 Reconnecting APU to audio...');
        this.audioManager.connectSystem(this.wasmSystem);
      } catch (error) {
        console.warn('Could not start audio:', error);
      }
    }
  }

  stop(): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.stop();

    // Stop audio playback
    if (this.audioManager) {
      this.audioManager.stop();
    }
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
      // Removed excessive logging - only log CPU state when needed for debug display
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

  // Filter control methods
  setFilterEnabled(enabled: boolean): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.set_filter_enabled(enabled);
  }

  setFilterCutoff(cutoff: number): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.set_filter_cutoff(cutoff);
  }

  setFilterResonance(resonance: number): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.set_filter_resonance(resonance);
  }

  setFilterType(filterType: number): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.set_filter_type(filterType);
  }

  getFilterCutoff(): number {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.get_filter_cutoff();
  }

  getFilterResonance(): number {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.get_filter_resonance();
  }

  getFilterType(): number {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.get_filter_type();
  }

  // Delay control methods
  setDelayEnabled(enabled: boolean): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.set_delay_enabled(enabled);
  }

  setDelayTime(delayTime: number): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.set_delay_time(delayTime);
  }

  setDelayFeedback(feedback: number): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.set_delay_feedback(feedback);
  }

  setDelayMix(mix: number): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.set_delay_mix(mix);
  }

  getDelayEnabled(): boolean {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.get_delay_enabled();
  }

  getDelayTime(): number {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.get_delay_time();
  }

  getDelayFeedback(): number {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.get_delay_feedback();
  }

  getDelayMix(): number {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.get_delay_mix();
  }

  // Demo melody control methods
  setMelodyEnabled(enabled: boolean): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.set_melody_enabled(enabled);
  }

  getMelodyEnabled(): boolean {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.get_melody_enabled();
  }

  setMelodyTempo(tempo: number): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.set_melody_tempo(tempo);
  }

  getMelodyTempo(): number {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.get_melody_tempo();
  }

  // Audio control methods
  setMasterVolume(volume: number): void {
    if (this.wasmSystem) {
      this.wasmSystem.set_master_volume(volume);
    }
    if (this.audioManager) {
      this.audioManager.setVolume(volume);
    }
  }

  getMasterVolume(): number {
    return this.audioManager ? this.audioManager.getVolume() : 0;
  }

  isAudioAvailable(): boolean {
    return this.audioManager ? this.audioManager.isAvailable() : false;
  }

  getAudioInfo(): any {
    return this.audioManager ? this.audioManager.getAudioInfo() : null;
  }

  // Debug method to test sample generation
  generateDebugSamples(count: number = 3): number[] {
    if (!this.wasmSystem) return [];

    const samples = [];
    for (let i = 0; i < count; i++) {
      try {
        samples.push(this.wasmSystem.generate_audio_sample());
      } catch (error) {
        console.error(`Error generating debug sample ${i}:`, error);
        samples.push(0);
      }
    }
    return samples;
  }
}

export class ZebratronCartridgeSystem {
  private wasmSystem: WasmCartridgeSystem | null = null;
  private canvas: HTMLCanvasElement | null = null;
  private ctx: CanvasRenderingContext2D | null = null;
  private audioManager: AudioManager | null = null;
  private midiManager: MidiManager | null = null;
  private isInitialized = false;

  async initialize(canvasElement?: HTMLCanvasElement): Promise<void> {
    if (this.isInitialized) return;

    // Initialize the WebAssembly module
    await init();
    this.wasmSystem = new WasmCartridgeSystem();

    // Set up canvas if provided
    if (canvasElement) {
      this.setupCanvas(canvasElement);
    }

    // Set up full audio system like original
    console.log('🎧 Initializing AudioManager for cartridge system...');
    this.audioManager = new AudioManager();
    await this.audioManager.initialize();

    console.log('🔗 Connecting WASM cartridge system to audio...');
    this.audioManager.connectSystem(this.wasmSystem);

    // Set up MIDI system
    console.log('🎹 Initializing MIDI support...');
    this.midiManager = new MidiManager();
    try {
      await this.midiManager.initialize();
      console.log('✅ MIDI system initialized');
      
      // Connect MIDI to Z-Synth
      this.midiManager.onNoteOn = (note: number, _velocity: number) => {
        if (this.wasmSystem && this.wasmSystem.get_current_cartridge_type() === 2) {
          this.wasmSystem.handle_midi_note_on(note);
        }
      };
      
      this.midiManager.onNoteOff = (note: number) => {
        if (this.wasmSystem && this.wasmSystem.get_current_cartridge_type() === 2) {
          this.wasmSystem.handle_midi_note_off(note);
        }
      };
      
    } catch (error) {
      console.warn('⚠️ MIDI initialization failed (may not be supported):', error);
    }

    // Load the Hambert cartridge and start
    this.wasmSystem.load_hambert_cartridge();
    this.wasmSystem.start();

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

  loadHambertCartridge(): boolean {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.load_hambert_cartridge();
  }

  async start(): Promise<void> {
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

    this.wasmSystem.render();

    const buffer = this.wasmSystem.get_screen_buffer();

    // Create ImageData from the WASM buffer
    const imageData = new ImageData(
      new Uint8ClampedArray(buffer),
      320,
      240
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
      return state;
    } catch (error) {
      console.error('Error getting CPU state:', error);
      return null;
    }
  }

  handleInput(up: boolean, down: boolean, left: boolean, right: boolean): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.handle_input(up, down, left, right);
  }

  toggleColorTest(): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.toggle_color_test();
  }

  getColorTestMode(): boolean {
    if (!this.wasmSystem) {
      return false;
    }
    return this.wasmSystem.get_color_test_mode();
  }

  // Simplified audio methods for cartridge system
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

  setMasterVolume(volume: number): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.set_master_volume(volume);
  }

  isAudioAvailable(): boolean {
    if (!this.wasmSystem) {
      return false;
    }
    return this.wasmSystem.is_audio_available();
  }

  getAudioInfo(): any {
    if (!this.wasmSystem) {
      return null;
    }
    return this.wasmSystem.get_audio_info();
  }

  generateDebugSamples(count: number = 3): number[] {
    if (!this.wasmSystem) {
      return [];
    }
    return Array.from(this.wasmSystem.generate_debug_samples(count));
  }

  // Filter controls (only work with Z-Synth cartridge - type 2)
  setFilterEnabled(enabled: boolean): void {
    if (!this.wasmSystem) return;
    if (this.wasmSystem.get_current_cartridge_type() === 2) {
      this.wasmSystem.set_filter_enabled(enabled);
    }
  }

  setFilterType(filterType: number): void {
    if (!this.wasmSystem) return;
    if (this.wasmSystem.get_current_cartridge_type() === 2) {
      this.wasmSystem.set_filter_type(filterType);
    }
  }

  setFilterCutoff(cutoff: number): void {
    if (!this.wasmSystem) return;
    if (this.wasmSystem.get_current_cartridge_type() === 2) {
      this.wasmSystem.set_filter_cutoff(cutoff);
    }
  }

  setFilterResonance(resonance: number): void {
    if (!this.wasmSystem) return;
    if (this.wasmSystem.get_current_cartridge_type() === 2) {
      this.wasmSystem.set_filter_resonance(resonance);
    }
  }

  // Delay controls
  setDelayEnabled(enabled: boolean): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.set_delay_enabled(enabled);
  }

  setDelayTime(delayTime: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.set_delay_time(delayTime);
  }

  setDelayFeedback(feedback: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.set_delay_feedback(feedback);
  }

  setDelayMix(mix: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.set_delay_mix(mix);
  }

  // Melody controls
  setMelodyEnabled(enabled: boolean): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.set_melody_enabled(enabled);
  }

  getMelodyEnabled(): boolean {
    if (!this.wasmSystem) return false;
    return this.wasmSystem.get_melody_enabled();
  }

  getFrameCount(): number {
    if (!this.wasmSystem) return 0;
    return Number(this.wasmSystem.get_frame_count());
  }

  // Z-Synth cartridge methods
  loadZSynthCartridge(): boolean {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    return this.wasmSystem.load_zsynth_cartridge();
  }

  handleZSynthKeyDown(key: string): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.handle_zsynth_key_down(key);
  }

  handleZSynthKeyUp(key: string): void {
    if (!this.wasmSystem) {
      throw new Error('System not initialized');
    }
    this.wasmSystem.handle_zsynth_key_up(key);
  }

  getCurrentCartridgeType(): number {
    if (!this.wasmSystem) return 0;
    return this.wasmSystem.get_current_cartridge_type();
  }

  getZSynthInfo(): string {
    if (!this.wasmSystem) return 'System not initialized';
    return this.wasmSystem.get_zsynth_info();
  }

  // MIDI support methods
  getMidiManager(): MidiManager | null {
    return this.midiManager;
  }

  isMidiAvailable(): boolean {
    return this.midiManager ? this.midiManager.isAvailable() : false;
  }

  getMidiInfo(): any {
    return this.midiManager ? this.midiManager.getMidiInfo() : null;
  }

  // Manual MIDI control (for testing or other integrations)
  handleMidiNoteOn(note: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.handle_midi_note_on(note);
  }

  handleMidiNoteOff(note: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.handle_midi_note_off(note);
  }

  // Emergency stop for MIDI
  midiAllNotesOff(): void {
    if (this.midiManager) {
      this.midiManager.allNotesOff();
    }
  }

  // SID-style 3-voice API for game developers
  sid_voice1_play_note(note: number, waveform: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.sid_voice1_play_note(note, waveform);
  }

  sid_voice2_play_note(note: number, waveform: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.sid_voice2_play_note(note, waveform);
  }

  sid_voice3_play_note(note: number, waveform: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.sid_voice3_play_note(note, waveform);
  }

  sid_voice1_stop(): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.sid_voice1_stop();
  }

  sid_voice2_stop(): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.sid_voice2_stop();
  }

  sid_voice3_stop(): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.sid_voice3_stop();
  }

  sid_stop_all(): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.sid_stop_all();
  }

  // SID filter controls (shared across all 3 voices)
  sid_set_filter_voices(voice1: boolean, voice2: boolean, voice3: boolean): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.sid_set_filter_voices(voice1, voice2, voice3);
  }

  sid_set_filter_cutoff(cutoff: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.sid_set_filter_cutoff(cutoff);
  }

  sid_set_filter_resonance(resonance: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.sid_set_filter_resonance(resonance);
  }

  sid_set_filter_type(filterType: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.sid_set_filter_type(filterType);
  }

  // Volume control for mixing SID and polyphonic layers
  set_sid_volume(volume: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.set_sid_volume(volume);
  }

  set_poly_volume(volume: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.set_poly_volume(volume);
  }

  // Polyphonic layer API (enhanced Z-Synth access)
  poly_play_chord(notes: number[]): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.poly_play_chord(new Uint8Array(notes));
  }

  poly_play_note(note: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.poly_play_note(note);
  }

  poly_stop_note(note: number): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.poly_stop_note(note);
  }

  poly_stop_all(): void {
    if (!this.wasmSystem) return;
    this.wasmSystem.poly_stop_all();
  }
}