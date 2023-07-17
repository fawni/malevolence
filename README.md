# Malevolence 🥀

Unofficial Rust bindings for the [OpenDota](https://www.opendota.com) API

Malevolence stands for [Orchid Malevolence](https://dota2.fandom.com/wiki/Orchid_Malevolence). "Orchid" is squatted by a [non-existent crate](https://crates.io/crates/orchid).

# Example

```rs
use malevolence::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(None);
    let dota_match = client.get_match(7_242_624_734).await?;
    println!("{:?}", dota_match.winner());

    Ok(())
}
```

# Roadmap

<details>
<summary>Roadmap</summary>

- [ ] Matches
  - [ ] `GET /matches/{match_id}`
- [ ] Players By Rank
  - [ ] `GET /playersByRank`
- [ ] Players
  - [ ] `GET /players/{account_id}`
  - [ ] `GET /players/{account_id}/wl`
  - [ ] `GET /players/{account_id}/recentMatches`
  - [ ] `GET /players/{account_id}/matches`
  - [ ] `GET /players/{account_id}/heroes`
  - [ ] `GET /players/{account_id}/peers`
  - [ ] `GET /players/{account_id}/pros`
  - [ ] `GET /players/{account_id}/totals`
  - [ ] `GET /players/{account_id}/counts`
  - [ ] `GET /players/{account_id}/histograms`
  - [ ] `GET /players/{account_id}/wardmap`
  - [ ] `GET /players/{account_id}/wordcloud`
  - [ ] `GET /players/{account_id}/ratings`
  - [ ] `GET /players/{account_id}/rankings`
  - [ ] `POST /players/{account_id}/refresh`
- [ ] Pro players
  - [ ] `GET /proPlayers`
- [ ] Pro matches
  - [ ] `GET /proMatches`
- [ ] Public matches
  - [ ] `GET /publicMatches`
- [ ] Parsed matches
  - [ ] `GET /parsedMatches`
- [ ] Distributions
  - [ ] `GET /distributions`
- [ ] Search
  - [ ] `GET /search`
- [ ] Rankings
  - [ ] `GET /rankings`
- [ ] Benchmarks
  - [ ] `GET /benchmarks`
- [ ] Find Matches
  - [ ] `GET /findMatches`
- [ ] Heroes
  - [ ] `GET /heroes`
  - [ ] `GET /heroes/{hero_id}/matches`
  - [ ] `GET /heroes/{hero_id}/matchups`
  - [ ] `GET /heroes/{hero_id}/durations`
  - [ ] `GET /heroes/{hero_id}/players`
  - [ ] `GET /heroes/{hero_id}/itemPo`
- [ ] Hero stats
  - [ ] `GET /heroStats`
- [ ] Leagues
  - [ ] `GET /leagues`
  - [ ] `GET /leagues/{league_id}`
  - [ ] `GET /leagues/{league_id}/matches`
  - [ ] `GET /leagues/{league_id}/teams`
- [ ] Teams
  - [ ] `GET /teams`
  - [ ] `GET /teams/{team_id}`
  - [ ] `GET /teams/{team_id}/matches`
  - [ ] `GET /teams/{team_id}/players`
  - [ ] `GET /teams/{team_id}/heroes`
- [ ] Replays
  - [ ] `GET /replays`
- [ ] Live
  - [ ] `GET /live`
- [ ] Scenarios
  - [ ] `GET /scenarios/itemTimings`
  - [ ] `GET /scenarios/laneRoles`
  - [ ] `GET /scenarios/misc`

</details>

# License

[OSL-3.0](LICENSE)