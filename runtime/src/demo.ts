import { ZebratronSystem, InputManager, Button } from './index.js';

class Demo {
  private system: ZebratronSystem;
  // private renderer: Renderer;
  private input: InputManager;
  private isRunning = false;
  private frameCount = 0;
  private lastTime = 0;
  private colorTestPressed = false;

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

    // Toggle color test mode with 'C' key
    if (this.input.isPressed(Button.Start)) { // Using Start button for demo
      // Only toggle once per press (simple debouncing)
      if (!this.colorTestPressed) {
        this.system.toggleColorTest();
        this.colorTestPressed = true;
      }
    } else {
      this.colorTestPressed = false;
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

  private updateDebugInfo(): void {
    const cpuStateElement = document.getElementById('cpuState');
    if (cpuStateElement) {
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

// Initialize the demo when the page loads
const demo = new Demo();
demo.initialize().then(() => {
  console.log('Demo ready!');
}).catch((error) => {
  console.error('Failed to initialize demo:', error);
});