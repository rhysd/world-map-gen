import { Board } from 'world-map-gen';

export interface Legend {
    text: string;
    color: string;
}
export interface Rendered {
    legends: Map<number, Legend>;
}
export interface Renderer {
    render(board: Board): Rendered;
}
