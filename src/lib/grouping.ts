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

  for (const movie of movies) {
    // Skip if no video type info
    if (movie.video_type === 'tv' || (movie.season && movie.episode)) {
      // Extract base title for grouping
      const baseTitle = extractSeriesKey(movie.cnname || movie.filename);
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
      const seasonDiff = parseInt(a.season || '0') - parseInt(b.season || '0');
      if (seasonDiff !== 0) return seasonDiff;
      return parseInt(a.episode || '0') - parseInt(b.episode || '0');
    });
  }

  return {
    series: Array.from(seriesMap.values()),
    individualMovies,
  };
}

function extractSeriesKey(title: string): string {
  return title
    .toLowerCase()
    .replace(/[._-]/g, ' ')
    .replace(/\s+/g, ' ')
    .replace(/season \d+|s\d+|第[一二三四五六七八九十\d]+季/gi, '')
    .trim();
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

// Remove duplicate movies by file hash or path
export function removeDuplicateMovies(movies: Movie[]): Movie[] {
  const seenPaths = new Set<string>();
  const seenHashes = new Set<string>();
  const uniqueMovies: Movie[] = [];

  for (const movie of movies) {
    // Skip if path already seen
    if (seenPaths.has(movie.path)) {
      continue;
    }

    // Skip if hash exists and already seen
    if (movie.file_hash && seenHashes.has(movie.file_hash)) {
      continue;
    }

    seenPaths.add(movie.path);
    if (movie.file_hash) {
      seenHashes.add(movie.file_hash);
    }
    uniqueMovies.push(movie);
  }

  return uniqueMovies;
}
