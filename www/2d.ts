import { Board } from 'world-map-gen';
import Renderer from './renderer';

export default class Renderer2D implements Renderer {
    canvas: HTMLCanvasElement;
    ctx: CanvasRenderingContext2D;

    constructor(root: HTMLElement) {
        this.canvas = document.createElement('canvas');
        this.canvas.className = 'screen';
        root.appendChild(this.canvas);

        this.ctx = this.canvas.getContext('2d')!;
    }

    render(board: Board) {
        const dpr = window.devicePixelRatio || 1;
        const rect = this.canvas.getBoundingClientRect();
        this.canvas.width = rect.width * dpr;
        this.canvas.height = rect.height * dpr;

        // Clear at first
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

        this.ctx.beginPath();

        const width = board.width;
        const height = board.height;
        const cellWidth = this.canvas.width / width;
        const cellHeight = this.canvas.height / height;

        const colors = new Map();
        for (let x = 0; x < width; x++) {
            for (let y = 0; y < height; y++) {
                const cell = board.at(x, y);
                const kind = cell.kind;
                let color = colors.get(kind);
                if (color === undefined) {
                    color = board.land_color_code(kind);
                    colors.set(kind, color);
                }
                this.ctx.fillStyle = color;
                this.ctx.fillRect(x * cellWidth, y * cellHeight, cellWidth, cellHeight);
            }
        }

        this.ctx.stroke();
    }
}
