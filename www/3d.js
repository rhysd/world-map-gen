import { Point3D, Point, CubeDimension, CubeColor, Cube, PixelView } from 'obelisk.js';
import * as obelisk from 'obelisk.js';

const CELL_SIZE = 10; // TODO: Temporary

export default class Renderer3D {
    constructor(root) {
        this.canvas = document.createElement('canvas');
        this.canvas.className = 'screen';
        root.appendChild(this.canvas);
    }

    determineCellSize(width, height, dpr) {
        const both = height + width;
        const fromHeight = ((this.canvas.height - 100) / dpr / both) * 2;
        const fromWidth = ((this.canvas.width / dpr / both) * 2) / Math.sqrt(3);
        let cellSize = Math.floor(fromHeight > fromWidth ? fromWidth : fromHeight);
        if (cellSize % 2 === 1) {
            cellSize--;
        }
        return cellSize > 6 ? cellSize : 6;
    }

    render(board) {
        const dpr = window.devicePixelRatio || 1;
        const rect = this.canvas.getBoundingClientRect();
        this.canvas.width = rect.width * dpr;
        this.canvas.height = rect.height * dpr;

        const width = board.width;
        const height = board.height;
        const cellSize = this.determineCellSize(width, height, dpr);

        const point = new Point(this.canvas.width / 2, cellSize + 99);
        const pixelView = new PixelView(this.canvas, point);

        const colors = new Map();
        for (let x = 0; x < width; x++) {
            for (let y = 0; y < height; y++) {
                const cell = board.at(x, y);
                const kind = cell.kind;
                let color = colors.get(kind);
                if (color === undefined) {
                    let rgb = board.land_rgb_color(kind);
                    if (rgb === undefined) {
                        rgb = 0xffffff;
                    }
                    color = new CubeColor().getByHorizontalColor(rgb);
                    colors.set(kind, color);
                }
                const height = cellSize + cell.altitude;
                const dim = new CubeDimension(cellSize, cellSize, height);
                const cube = new Cube(dim, color, /*border:*/ false);
                const pt = new Point3D(x * cellSize, y * cellSize, 0);
                pixelView.renderObject(cube, pt);
            }
        }
    }
}
