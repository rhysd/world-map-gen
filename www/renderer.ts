import { Board } from 'world-map-gen';

export interface Legend {
    text: string;
    color: string | undefined;
}
export interface Rendered {
    legends: Map<number, Legend>;
}
export interface Renderer {
    render(board: Board): Rendered;
}
