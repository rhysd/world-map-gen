import { Point3D, Point, CubeDimension, CubeColor, Cube, PixelView } from 'obelisk.js';
import { LandKind, Board, Cell } from 'world-map-gen';

export default class Renderer3D {
    canvas: HTMLCanvasElement;

    constructor(root: HTMLElement) {
        this.canvas = document.createElement('canvas');
        this.canvas.className = 'screen';
        root.appendChild(this.canvas);
    }

    determineCellSize(width: number, height: number) {
        const both = height + width;
        const fromHeight = ((this.canvas.height - 200) / both) * 2;
        const fromWidth = ((this.canvas.width / both) * 2) / Math.sqrt(3);
        let cellSize = Math.floor(fromHeight > fromWidth ? fromWidth : fromHeight);
        if (cellSize % 2 === 1) {
            cellSize--;
        }
        return cellSize > 6 ? cellSize : 6;
    }

    render(board: Board) {
        const dpr = window.devicePixelRatio || 1;
        const rect = this.canvas.getBoundingClientRect();
        this.canvas.width = rect.width * dpr;
        this.canvas.height = rect.height * dpr;

        const width = board.width;
        const height = board.height;
        const cellSize = this.determineCellSize(width, height);

        const point = new Point(this.canvas.width / 2, cellSize + 99 * 2);
        const pixelView = new PixelView(this.canvas, point);

        const cache = new Map();
        const colors = new Map();

        function kindColor(kind: LandKind): CubeColor {
            const cached = colors.get(kind);
            if (cached !== undefined) {
                return cached;
            }
            let rgb = board.land_rgb_color(kind);
            if (rgb === undefined) {
                rgb = 0xffffff;
            }
            const color = new CubeColor().getByHorizontalColor(rgb);
            colors.set(kind, color);
            return color;
        }

        function calcCube(kind: LandKind, alt: number): Cube {
            const color = kindColor(kind);
            const height = cellSize + alt * 2;
            const dim = new CubeDimension(cellSize, cellSize, height);
            return new Cube(dim, color, /*border:*/ false);
        }

        function cubeAt(cell: Cell): Cube {
            const kind = cell.kind;
            const alt = cell.altitude;

            if (kind === LandKind.Town || kind === LandKind.Path) {
                return calcCube(kind, alt);
            }

            const cached = cache.get(alt);
            if (cached !== undefined) {
                return cached;
            }

            const cube = calcCube(kind, alt);
            cache.set(alt, cube);
            return cube;
        }

        for (let x = 0; x < width; x++) {
            for (let y = 0; y < height; y++) {
                const cube = cubeAt(board.at(x, y));
                const pt = new Point3D(x * cellSize, y * cellSize, 0);
                pixelView.renderObject(cube, pt);
            }
        }
    }
}
