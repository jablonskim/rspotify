//! All kinds of playlists objects
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::image::Image;
use super::page::Page;
use super::track::FullTrack;
use super::user::PublicUser;
use crate::model::Type;

///[Playlist object simplified](https://developer.spotify.com/documentation/web-api/reference/object-model/#playlist-object-simplified)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedPlaylist {
    pub collaborative: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: HashMap<String, Value>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FullPlaylist {
    pub collaborative: bool,
    pub description: String,
    pub external_urls: HashMap<String, String>,
    pub followers: Option<HashMap<String, Value>>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: Page<PlaylistItem>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// [Playlist track object](https://developer.spotify.com/documentation/web-api/reference/object-model/#playlist-track-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlaylistItem {
    pub added_at: Option<DateTime<Utc>>,
    pub added_by: Option<PublicUser>,
    pub is_local: bool,
    pub track: Option<FullTrack>,
}
/// [Get list featured playlists](https://developer.spotify.com/web-api/get-list-featured-playlists/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FeaturedPlaylists {
    pub message: String,
    pub playlists: Page<SimplifiedPlaylist>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_playlist_item() {
        let json_str = r#"
    {
        "added_at": "2015-01-15T12:39:22Z",
        "added_by": {
            "external_urls": {
            "spotify": "https://open.spotify.com/user/jmperezperez"
            },
            "href": "https://api.spotify.com/v1/users/jmperezperez",
            "id": "jmperezperez",
            "type": "user",
            "uri": "spotify:user:jmperezperez"
        },
        "is_local": false,
        "primary_color": null,
        "track": {
            "album": {
            "album_type": "album",
            "artists": [
                {
                "external_urls": {
                    "spotify": "https://open.spotify.com/artist/5I8r2w4hf7OYp2cunjihxJ"
                },
                "href": "https://api.spotify.com/v1/artists/5I8r2w4hf7OYp2cunjihxJ",
                "id": "5I8r2w4hf7OYp2cunjihxJ",
                "name": "Kularis",
                "type": "artist",
                "uri": "spotify:artist:5I8r2w4hf7OYp2cunjihxJ"
                }
            ],
            "external_urls": {
                "spotify": "https://open.spotify.com/album/2pANdqPvxInB0YvcDiw4ko"
            },
            "href": "https://api.spotify.com/v1/albums/2pANdqPvxInB0YvcDiw4ko",
            "id": "2pANdqPvxInB0YvcDiw4ko",
            "images": [
                {
                "height": 640,
                "url": "https://i.scdn.co/image/ab67616d0000b273ce6d0eef0c1ce77e5f95bbbc",
                "width": 640
                },
                {
                "height": 300,
                "url": "https://i.scdn.co/image/ab67616d00001e02ce6d0eef0c1ce77e5f95bbbc",
                "width": 300
                },
                {
                "height": 64,
                "url": "https://i.scdn.co/image/ab67616d00004851ce6d0eef0c1ce77e5f95bbbc",
                "width": 64
                }
            ],
            "name": "Progressive Psy Trance Picks Vol.8",
            "release_date": "2012-04-02",
            "release_date_precision": "day",
            "total_tracks": 20,
            "type": "album",
            "uri": "spotify:album:2pANdqPvxInB0YvcDiw4ko"
            },
            "artists": [
            {
                "external_urls": {
                "spotify": "https://open.spotify.com/artist/6eSdhw46riw2OUHgMwR8B5"
                },
                "href": "https://api.spotify.com/v1/artists/6eSdhw46riw2OUHgMwR8B5",
                "id": "6eSdhw46riw2OUHgMwR8B5",
                "name": "Odiseo",
                "type": "artist",
                "uri": "spotify:artist:6eSdhw46riw2OUHgMwR8B5"
            }
            ],
            "disc_number": 1,
            "duration_ms": 376000,
            "episode": false,
            "explicit": false,
            "external_ids": {
            "isrc": "DEKC41200989"
            },
            "external_urls": {
            "spotify": "https://open.spotify.com/track/4rzfv0JLZfVhOhbSQ8o5jZ"
            },
            "href": "https://api.spotify.com/v1/tracks/4rzfv0JLZfVhOhbSQ8o5jZ",
            "id": "4rzfv0JLZfVhOhbSQ8o5jZ",
            "is_local": false,
            "is_playable": true,
            "name": "Api",
            "popularity": 2,
            "preview_url": "https://p.scdn.co/mp3-preview/c440fa9ff320fb74629f8477bff45b1a377897ab?cid=774b29d4f13844c495f206cafdad9c86",
            "track": true,
            "track_number": 10,
            "type": "track",
            "uri": "spotify:track:4rzfv0JLZfVhOhbSQ8o5jZ"
        },
        "video_thumbnail": {
            "url": null
        }
        }
        "#;
        let playlist_item: PlaylistItem = serde_json::from_str(&json_str).unwrap();
        assert_eq!(
            playlist_item.added_by.unwrap().id,
            "jmperezperez".to_string()
        );
    }
}
