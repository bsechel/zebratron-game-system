/**
 * Web MIDI API integration for ZebratronGameSystem
 * Allows external MIDI controllers and DAWs to control the Z-Synth
 */

export class MidiManager {
  private midiAccess: MIDIAccess | null = null;
  private isInitialized = false;
  private midiInputs: Map<string, MIDIInput> = new Map();
  private midiOutputs: Map<string, MIDIOutput> = new Map();
  private activeNotes: Set<number> = new Set();
  
  // Callback functions that can be set by the Z-Synth
  public onNoteOn: ((note: number, velocity: number) => void) | null = null;
  public onNoteOff: ((note: number) => void) | null = null;
  public onControlChange: ((controller: number, value: number) => void) | null = null;

  constructor() {}

  /**
   * Initialize Web MIDI API
   * Must be called after user interaction due to browser permissions
   */
  async initialize(): Promise<void> {
    if (this.isInitialized) return;

    try {
      // Check if Web MIDI is supported
      if (!navigator.requestMIDIAccess) {
        throw new Error('Web MIDI API not supported in this browser');
      }

      console.log('🎹 Requesting MIDI access...');
      
      // Request MIDI access (sysex: false for basic note/CC support)
      this.midiAccess = await navigator.requestMIDIAccess({ sysex: false });
      
      console.log('✅ MIDI access granted');

      // Set up input and output device monitoring
      this.setupDeviceMonitoring();
      
      // Scan for existing devices
      this.scanDevices();

      this.isInitialized = true;
      console.log('🎛️ MIDI system initialized successfully');
      
    } catch (error) {
      console.error('❌ Failed to initialize MIDI:', error);
      throw error;
    }
  }

  /**
   * Set up monitoring for MIDI device connections/disconnections
   */
  private setupDeviceMonitoring(): void {
    if (!this.midiAccess) return;

    this.midiAccess.onstatechange = (event: any) => {
      const port = event.port;
      console.log(`🔌 MIDI device ${port.state}: ${port.name} (${port.type})`);
      
      if (port.state === 'connected') {
        if (port.type === 'input') {
          this.connectInput(port as MIDIInput);
        } else if (port.type === 'output') {
          this.connectOutput(port as MIDIOutput);
        }
      } else if (port.state === 'disconnected') {
        this.disconnectPort(port);
      }
    };
  }

  /**
   * Scan for existing MIDI devices
   */
  private scanDevices(): void {
    if (!this.midiAccess) return;

    console.log('🔍 Scanning for MIDI devices...');

    // Scan inputs
    this.midiAccess.inputs.forEach((input: MIDIInput) => {
      this.connectInput(input);
    });

    // Scan outputs  
    this.midiAccess.outputs.forEach((output: MIDIOutput) => {
      this.connectOutput(output);
    });

    console.log(`📥 Found ${this.midiInputs.size} MIDI input(s)`);
    console.log(`📤 Found ${this.midiOutputs.size} MIDI output(s)`);
  }

  /**
   * Connect a MIDI input device
   */
  private connectInput(input: MIDIInput): void {
    if (this.midiInputs.has(input.id)) return;

    console.log(`📥 Connecting MIDI input: ${input.name}`);
    
    this.midiInputs.set(input.id, input);
    
    // Set up message handler
    input.onmidimessage = (event: any) => {
      this.handleMidiMessage(event);
    };

    input.onstatechange = (event: any) => {
      console.log(`📥 Input ${input.name} state: ${event.port.state}`);
    };
  }

  /**
   * Connect a MIDI output device
   */
  private connectOutput(output: MIDIOutput): void {
    if (this.midiOutputs.has(output.id)) return;

    console.log(`📤 Connecting MIDI output: ${output.name}`);
    this.midiOutputs.set(output.id, output);
  }

  /**
   * Disconnect a MIDI port
   */
  private disconnectPort(port: MIDIPort): void {
    if (port.type === 'input') {
      this.midiInputs.delete(port.id);
      console.log(`📥 Disconnected MIDI input: ${port.name}`);
    } else if (port.type === 'output') {
      this.midiOutputs.delete(port.id);
      console.log(`📤 Disconnected MIDI output: ${port.name}`);
    }
  }

  /**
   * Handle incoming MIDI messages
   */
  private handleMidiMessage(event: any): void {
    const [status, data1, data2] = event.data;
    
    // Extract message type and channel
    const messageType = status & 0xF0;
    const channel = status & 0x0F;
    
    switch (messageType) {
      case 0x90: // Note On
        if (data2 > 0) { // Velocity > 0 means note on
          this.handleNoteOn(data1, data2);
        } else { // Velocity = 0 means note off
          this.handleNoteOff(data1);
        }
        break;
        
      case 0x80: // Note Off
        this.handleNoteOff(data1);
        break;
        
      case 0xB0: // Control Change
        this.handleControlChange(data1, data2);
        break;
        
      case 0xE0: // Pitch Bend
        const pitchBend = (data2 << 7) | data1;
        console.log(`🎛️ Pitch bend: ${pitchBend} (channel ${channel + 1})`);
        break;
        
      default:
        console.log(`🎛️ MIDI message: ${Array.from(event.data).map((b: any) => b.toString(16)).join(' ')}`);
    }
  }

  /**
   * Handle MIDI Note On
   */
  private handleNoteOn(note: number, velocity: number): void {
    console.log(`🎵 MIDI Note ON: ${note} (${this.noteToName(note)}) velocity: ${velocity}`);
    
    this.activeNotes.add(note);
    
    if (this.onNoteOn) {
      this.onNoteOn(note, velocity);
    }
  }

  /**
   * Handle MIDI Note Off
   */
  private handleNoteOff(note: number): void {
    console.log(`🎵 MIDI Note OFF: ${note} (${this.noteToName(note)})`);
    
    this.activeNotes.delete(note);
    
    if (this.onNoteOff) {
      this.onNoteOff(note);
    }
  }

  /**
   * Handle MIDI Control Change
   */
  private handleControlChange(controller: number, value: number): void {
    console.log(`🎛️ MIDI CC: ${controller} = ${value}`);
    
    if (this.onControlChange) {
      this.onControlChange(controller, value);
    }
  }

  /**
   * Convert MIDI note number to note name
   */
  private noteToName(note: number): string {
    const noteNames = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
    const octave = Math.floor(note / 12) - 1;
    const noteName = noteNames[note % 12];
    return `${noteName}${octave}`;
  }

  /**
   * Get list of available MIDI inputs
   */
  getInputDevices(): Array<{id: string, name: string}> {
    const devices: Array<{id: string, name: string}> = [];
    
    this.midiInputs.forEach((input, id) => {
      devices.push({
        id: id,
        name: input.name || 'Unknown Device'
      });
    });
    
    return devices;
  }

  /**
   * Get list of available MIDI outputs
   */
  getOutputDevices(): Array<{id: string, name: string}> {
    const devices: Array<{id: string, name: string}> = [];
    
    this.midiOutputs.forEach((output, id) => {
      devices.push({
        id: id,
        name: output.name || 'Unknown Device'
      });
    });
    
    return devices;
  }

  /**
   * Get currently active MIDI notes
   */
  getActiveNotes(): number[] {
    return Array.from(this.activeNotes);
  }

  /**
   * Send a MIDI note on message to all outputs
   */
  sendNoteOn(note: number, velocity: number = 100, channel: number = 0): void {
    const message = [0x90 | channel, note, velocity];
    this.sendToAllOutputs(message);
  }

  /**
   * Send a MIDI note off message to all outputs
   */
  sendNoteOff(note: number, channel: number = 0): void {
    const message = [0x80 | channel, note, 0];
    this.sendToAllOutputs(message);
  }

  /**
   * Send raw MIDI data to all outputs
   */
  private sendToAllOutputs(data: number[]): void {
    this.midiOutputs.forEach((output) => {
      try {
        output.send(data);
      } catch (error) {
        console.error(`Failed to send MIDI to ${output.name}:`, error);
      }
    });
  }

  /**
   * Emergency stop - turn off all active notes
   */
  allNotesOff(): void {
    console.log('🛑 MIDI All Notes Off');
    
    // Send note off for all currently active notes
    this.activeNotes.forEach(note => {
      this.handleNoteOff(note);
    });
    
    this.activeNotes.clear();
    
    // Send MIDI All Notes Off message (CC 123)
    for (let channel = 0; channel < 16; channel++) {
      const message = [0xB0 | channel, 123, 0];
      this.sendToAllOutputs(message);
    }
  }

  /**
   * Check if MIDI is available and initialized
   */
  isAvailable(): boolean {
    return this.isInitialized && this.midiAccess !== null;
  }

  /**
   * Get MIDI system info for debugging
   */
  getMidiInfo(): any {
    if (!this.midiAccess) return null;

    return {
      sysexEnabled: this.midiAccess.sysexEnabled,
      inputs: this.getInputDevices(),
      outputs: this.getOutputDevices(),
      activeNotes: this.getActiveNotes()
    };
  }

  /**
   * Cleanup MIDI resources
   */
  dispose(): void {
    // Stop all active notes
    this.allNotesOff();
    
    // Disconnect all inputs
    this.midiInputs.forEach((input) => {
      input.onmidimessage = null;
      input.onstatechange = null;
    });
    
    this.midiInputs.clear();
    this.midiOutputs.clear();
    
    if (this.midiAccess) {
      this.midiAccess.onstatechange = null;
    }
    
    this.midiAccess = null;
    this.isInitialized = false;
    
    console.log('🔌 MIDI system disposed');
  }
}