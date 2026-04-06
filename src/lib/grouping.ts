import type { Movie } from './api';

export interface GroupedSeries {
  key: string;
  title: string;
  year: string | null;
  poster_path: string | null;
  episodes: Movie[];
  seasonCount: number;
  episodeCount: number;
}

export interface GroupedMoviesResult {
  series: GroupedSeries[];
  individualMovies: Movie[];
}

export function groupMoviesBySeries(movies: Movie[]): GroupedMoviesResult {
  const seriesMap = new Map<string, GroupedSeries>();
  const individualMovies: Movie[] = [];

  // Pre-compile regex for better performance
  const seasonRegex = /season \d+|s\d+|第[一二三四五六七八九十\d]+季/gi;
  const separators = /[._-]/g;

  for (const movie of movies) {
    // Skip if no video type info
    if (movie.video_type === 'tv' || (movie.season && movie.episode)) {
      // Extract base title for grouping - optimized
      const title = movie.cnname || movie.filename;
      const baseTitle = title
        .toLowerCase()
        .replace(separators, ' ')
        .replace(/\s+/g, ' ')
        .replace(seasonRegex, '')
        .trim();

      const key = `${baseTitle}_${movie.year || 'unknown'}`;

      if (!seriesMap.has(key)) {
        seriesMap.set(key, {
          key,
          title: movie.cnname || extractSeriesTitle(movie.filename),
          year: movie.year,
          poster_path: movie.poster_path,
          episodes: [],
          seasonCount: 0,
          episodeCount: 0,
        });
      }

      const series = seriesMap.get(key)!;
      series.episodes.push(movie);
      series.episodeCount++;
      if (movie.season) {
        const seasonNum = parseInt(movie.season);
        if (!isNaN(seasonNum) && seasonNum > series.seasonCount) {
          series.seasonCount = seasonNum;
        }
      }
    } else {
      individualMovies.push(movie);
    }
  }

  // Sort episodes within each series
  for (const series of seriesMap.values()) {
    series.episodes.sort((a, b) => {
      const seasonA = parseInt(a.season || '0');
      const seasonB = parseInt(b.season || '0');
      if (seasonA !== seasonB) return seasonA - seasonB;
      return parseInt(a.episode || '0') - parseInt(b.episode || '0');
    });
  }

  return {
    series: Array.from(seriesMap.values()),
    individualMovies,
  };
}

function extractSeriesTitle(filename: string): string {
  // Remove file extension and common patterns
  return filename
    .replace(/\.[^.]+$/, '')
    .replace(/s\d+e\d+/gi, '')
    .replace(/season \d+/gi, '')
    .replace(/[._-]/g, ' ')
    .trim();
}

// Remove duplicate movies by movie identity (cnname + year + season + episode)
// Same movie/show appears only once in library, but multiple file versions are tracked
export function removeDuplicateMovies(movies: Movie[]): Movie[] {
  const seenMovies = new Set<string>();
  const uniqueMovies: Movie[] = [];

  for (let i = 0; i < movies.length; i++) {
    const movie = movies[i];

    // Create unique key based on movie identity - optimized
    const title = movie.cnname || movie.filename;
    const year = movie.year || 'unknown';
    const season = movie.season || '';
    const episode = movie.episode || '';

    // Build key more efficiently
    let key: string;
    if (movie.video_type === 'tv' || (season && episode)) {
      key = `${title}_${year}_S${season}E${episode}`.toLowerCase().trim();
    } else {
      key = `${title}_${year}`.toLowerCase().trim();
    }

    // Skip if this movie/show already seen
    if (seenMovies.has(key)) {
      continue;
    }

    seenMovies.add(key);
    uniqueMovies.push(movie);
  }

  return uniqueMovies;
}
