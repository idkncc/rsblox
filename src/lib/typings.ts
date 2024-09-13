import type { Writable } from "svelte/store";

// Svelte typings

export interface InternalUser {
    info: UserInformation;
    headshot: string;
    presence: UserPresence;
}

export type ClientInfoWritable = Writable<ClientInfo | undefined>

export interface ClientInfo {
    user_id: number;
    username: string;
    display_name: string;
    robux: number;
}

// Api Typings

export interface UserDetails {
    id: number,
    username: string,
    display_name: string,
    description: string,

    // A time string of when the account was created. Follows ISO 8061 format.
    created_at: string,

    // Whether the account is terminated. Does not include non-termination bans.
    is_terminated: boolean,
    has_verified_badge: boolean,
}

export interface UserProfileStats {
    friends: number,
    followers: number,
    followings: number
}

export interface UserInformation {
    user_id: number;
    username: string;

    display_name: string;

    description?: string;

    created: string;

    presence_type: string;

    // Whether the user is banned/terminated.
    is_terminated: boolean;

    // The user's verified badge status.
    has_verified_badge: boolean;
}

export enum FriendStatus {
    NotFriends = "NotFriends",
    Friends = "Friends",
    RequestSent = "RequestSent",
    RequestReceived = "RequestReceived",
}

export enum PresenceType {
    Offline = "Offline",
    Online = "Online",
    InGame = "InGame",
    InStudio = "InStudio",
    Invisible = "Invisible",
}

export interface UserPresence {
    user_id: number;
    presence_type: PresenceType;
    last_online: string;
    last_location: string;

    place_id?: number;
    game_id?: string;
    universe_id?: number;
}

export enum TreatmentType {
    FriendCarousel = "FriendCarousel",
    Carousel = "Carousel",
    SortlessGrid = "SortlessGrid",
}

export interface RecommendationsTopic {
    topic_id: number;

    /// Topic / Title
    topic?: string;

    /// Subtitle
    subtitle?: string;

    /// Type of topic
    treatment_type: TreatmentType;

    /// Array of recommendations
    recommendation_list: Array<Recommendation>;
}


export enum GameMediaType {
    Image = "Image",
    YoutubeVideo = "YoutubeVideo"
}

export interface GameMedia {
    asset_type_id: number,
    asset_type: GameMediaType,
    approved: boolean,

    image_id?: number,
    alt_text?: string,
    video_hash?: string,
    video_title?: string,
}

export interface GameServer {
    id: string,
    max_players: number,
    playing: number,

    player_tokens: string[],

    // pub players: Vec<String>,
    fps: number,
    ping: number,
}

export enum AvatarType {
    MorphToR6 = "MorphToR6",
    MorphToR15 = "MorphToR15",
    PlayerChoice = "PlayerChoice",
}

//// Universe/Game related: ////

/**
 * Base interface of Universe (aka Game, which stores places)
 */
export interface BaseUniverse {
    universe_id: number;
    root_place_id: number;
    name: string;
    description?: string;
}

export interface Recommendation extends BaseUniverse {
    total_up_votes: number;
    total_down_votes: number;
    player_count: number;
}

export interface SearchResult extends BaseUniverse {
    total_up_votes: number;
    total_down_votes: number;
    player_count: number;

    emphasis: boolean,
    is_sponsored: boolean,

    creator_id: number,
    creator_name: string,
    creator_has_verified_badge: boolean,

    minimum_age: number,
    age_recommendation_display_name: string,

    content_type: string,
    content_id: number,
}

export interface GameDetails extends BaseUniverse {
    /// Original game's name
    source_name: string;

    /// Original game's description
    source_description: string;

    creator: any;

    price?: number;

    allowed_gear_genres: string[];
    allowed_gear_categories: string[];
    is_genre_enforced: boolean;
    copying_allowed: boolean;

    playing: number;
    visits: number;
    max_players: number;
    created: string;
    updated: string;

    /// Avatar type. Possible values are MorphToR6, MorphToR15, and PlayerChoice
    universe_avatar_type: string;

    genre: String;
    is_all_genre: boolean;

    is_favorited_by_user: boolean;
    favorited_count: number;
}

export interface PlaceDetails {
    place_id: number;

    name: string;
    description: string;
    source_name: string;
    source_description: string;

    url: string;

    is_playable: boolean;
    reason_prohibited: string;
    price: number;
    image_token: string;

    builder: string;
    builder_id: number;
    has_verified_badge: boolean;

    universe_id: number;
    universe_root_place_id: number;
}

export const enum ThumbnailSize {
    S30x30 = "S30x30",
    S42x42 = "S42x42",
    S50x50 = "S50x50",
    S60x62 = "S60x62",
    S75x75 = "S75x75",
    S110x110 = "S110x110",
    S140x140 = "S140x140",
    S150x150 = "S150x150",
    S160x100 = "S160x100",
    S160x600 = "S160x600",
    S250x250 = "S250x250",
    S256x144 = "S256x144",
    S300x250 = "S300x250",
    S304x166 = "S304x166",
    S384x216 = "S384x216",
    S396x216 = "S396x216",
    S420x420 = "S420x420",
    S480x270 = "S480x270",
    S512x512 = "S512x512",
    S576x324 = "S576x324",
    S700x700 = "S700x700",
    S728x90 = "S728x90",
    S768x432 = "S768x432",
    S1200x80 = "S1200x80",
}

export const enum ThumbnailType {
    Avatar = "Avatar",
    AvatarHeadshot = "AvatarHeadshot",
    Asset = "Asset",
    GameIcon = "GameIcon",
    GameThumbnail = "GameThumbnail",
}

export interface TrayGame {
    id: number,
    title: string
}
