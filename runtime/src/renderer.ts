export class Renderer {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private scale: number;

  constructor(canvas: HTMLCanvasElement, scale: number = 2) {
    this.canvas = canvas;
    this.scale = scale;

    const ctx = canvas.getContext('2d');
    if (!ctx) {
      throw new Error('Failed to get 2D rendering context');
    }
    this.ctx = ctx;

    // Set up canvas
    this.setupCanvas();
  }

  private setupCanvas(): void {
    // Set internal resolution
    this.canvas.width = 320;
    this.canvas.height = 240;

    // Set display size with scaling
    this.canvas.style.width = `${320 * this.scale}px`;
    this.canvas.style.height = `${240 * this.scale}px`;

    // Disable image smoothing for crisp pixel art
    this.ctx.imageSmoothingEnabled = false;
  }

  renderFrame(buffer: Uint8Array): void {
    const imageData = new ImageData(
      new Uint8ClampedArray(buffer),
      320,
      240
    );
    this.ctx.putImageData(imageData, 0, 0);
  }

  clear(): void {
    this.ctx.fillStyle = '#202040';
    this.ctx.fillRect(0, 0, 320, 240);
  }

  setScale(scale: number): void {
    this.scale = scale;
    this.canvas.style.width = `${320 * scale}px`;
    this.canvas.style.height = `${240 * scale}px`;
  }

  getScale(): number {
    return this.scale;
  }
}