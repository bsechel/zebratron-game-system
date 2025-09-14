export enum Button {
  Up = 0,
  Down = 1,
  Left = 2,
  Right = 3,
  A = 4,
  B = 5,
  Select = 6,
  Start = 7,
}

export class InputManager {
  private buttonStates: boolean[] = new Array(8).fill(false);
  private keyBindings: Map<string, Button> = new Map();

  constructor() {
    this.setupDefaultBindings();
    this.setupEventListeners();
  }

  private setupDefaultBindings(): void {
    // Arrow keys for D-pad
    this.keyBindings.set('ArrowUp', Button.Up);
    this.keyBindings.set('ArrowDown', Button.Down);
    this.keyBindings.set('ArrowLeft', Button.Left);
    this.keyBindings.set('ArrowRight', Button.Right);

    // WASD for D-pad (alternative)
    this.keyBindings.set('KeyW', Button.Up);
    this.keyBindings.set('KeyS', Button.Down);
    this.keyBindings.set('KeyA', Button.Left);
    this.keyBindings.set('KeyD', Button.Right);

    // Action buttons
    this.keyBindings.set('KeyZ', Button.A);
    this.keyBindings.set('KeyX', Button.B);
    this.keyBindings.set('Space', Button.A);
    this.keyBindings.set('ShiftLeft', Button.B);

    // Menu buttons
    this.keyBindings.set('Tab', Button.Select);
    this.keyBindings.set('Enter', Button.Start);
  }

  private setupEventListeners(): void {
    document.addEventListener('keydown', (event) => {
      const button = this.keyBindings.get(event.code);
      if (button !== undefined) {
        this.buttonStates[button] = true;
        event.preventDefault();
      }
    });

    document.addEventListener('keyup', (event) => {
      const button = this.keyBindings.get(event.code);
      if (button !== undefined) {
        this.buttonStates[button] = false;
        event.preventDefault();
      }
    });
  }

  isPressed(button: Button): boolean {
    return this.buttonStates[button];
  }

  getButtonState(): number {
    // Pack button states into a single byte
    let state = 0;
    for (let i = 0; i < 8; i++) {
      if (this.buttonStates[i]) {
        state |= (1 << i);
      }
    }
    return state;
  }

  setKeyBinding(key: string, button: Button): void {
    this.keyBindings.set(key, button);
  }

  clearKeyBinding(key: string): void {
    this.keyBindings.delete(key);
  }

  getBindings(): Map<string, Button> {
    return new Map(this.keyBindings);
  }
}