import { Board } from 'world-map-gen';

interface Renderer {
    render(board: Board): void;
}

export default Renderer;
