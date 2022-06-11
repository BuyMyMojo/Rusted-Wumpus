// I wounder if storing this text as a const is more efficient then just putting it inside the reply function? I will ask around later.
pub const INFO_MESSAGE: &str = "
Hello there, Human!

This is just an example message I am making as a test for this bot!

So far I've added a multi thread test command, OwOfication command, a couple AniList lookup commands, age of discord ID command and a few others!

I'll keep adding more till the end of time.

— RustBot 🤖🎉🦀
";

// Query to use in AniList request
pub const ANIME_QUERY: &str = "
query ($search: String) { # Define which variables will be used in the query (id)
  Media (search: $search, type: ANIME) { # Insert our variables into the query arguments (id) (type: ANIME is hard-coded in the query)
    id
    title {
      romaji
      english
      native
    }
    status
    description
    startDate {
        year
        month
        day
    }
    endDate {
        year
        month
        day
    }
    coverImage {
        extraLarge
        large
        color
    }
    season
    seasonYear
    seasonInt
    episodes
    duration
    hashtag
    trailer {
        id
        site
        thumbnail
    }
    genres
    averageScore
    meanScore
    isAdult
    siteUrl
  }
}
";

// Query to use in AniList request
pub const MANGA_QUERY: &str = "
query ($search: String) { # Define which variables will be used in the query (id)
  Media (search: $search, type: MANGA) { # Insert our variables into the query arguments (id) (type: MANGA is hard-coded in the query)
    id
    title {
      romaji
      english
      native
    }
    status
    description
    startDate {
        year
        month
        day
    }
    endDate {
        year
        month
        day
    }
    coverImage {
        extraLarge
        large
        color
    }
    volumes
    chapters
    season
    seasonYear
    seasonInt
    hashtag
    genres
    averageScore
    meanScore
    isAdult
    siteUrl
  }
}
";
