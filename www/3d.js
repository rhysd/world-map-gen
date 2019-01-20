import { Point3D, Point, CubeDimension, CubeColor, Cube, PixelView } from 'obelisk.js';
import * as obelisk from 'obelisk.js';

const CELL_SIZE = 10; // TODO: Temporary

export default class Renderer3D {
    constructor(root) {
        this.canvas = document.createElement('canvas');
        this.canvas.className = 'screen';
        root.appendChild(this.canvas);
    }

    render(board) {
        const dpr = window.devicePixelRatio || 1;
        const rect = this.canvas.getBoundingClientRect();
        this.canvas.width = rect.width * dpr;
        this.canvas.height = rect.height * dpr;

        const point = new Point(this.canvas.width / 2, 200);
        const pixelView = new PixelView(this.canvas, point);

        const width = board.width;
        const height = board.height;

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
                const height = CELL_SIZE + cell.altitude;
                const dim = new CubeDimension(CELL_SIZE, CELL_SIZE, height);
                const cube = new Cube(dim, color, false);
                const pt = new Point3D(x * CELL_SIZE, y * CELL_SIZE, 0);
                pixelView.renderObject(cube, pt);
            }
        }
    }
}
