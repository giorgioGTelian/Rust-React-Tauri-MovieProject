import React, { useEffect, useState } from 'react';
import { getMovies, Movie } from './api';

function App() {
  const [movies, setMovies] = useState<Movie[]>([]);

  useEffect(() => {
    const loadMovies = async () => {
      try {
        const data = await getMovies();
        setMovies(data);
      } catch (error) {
        console.error('Error loading movies:', error);
      }
    };
    loadMovies();
  }, []);

  return (
    <div className="container">
      <h1>My Movie Collection</h1>
      <div className="movie-grid">
        {movies.map(movie => (
          <div key={movie.id} className="movie-card">
            {movie.poster_path && (
              <img 
                src={movie.poster_path} 
                alt={movie.title}
                onError={(e) => {
                  (e.target as HTMLImageElement).style.display = 'none';
                }}
              />
            )}
            <h3>{movie.title} {movie.year && `(${movie.year})`}</h3>
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;