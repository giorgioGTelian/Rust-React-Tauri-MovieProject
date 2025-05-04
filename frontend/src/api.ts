import { invoke } from '@tauri-apps/api/core';

export const getMovies = async () => {
    return await invoke<Movie[]>('get_movies');
};

export type Movie = {
    id: number;
    title: string;
    year?: number;
    poster_path?: string;
};