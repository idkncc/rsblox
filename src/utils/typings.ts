export interface FriendUserInformation {
    user_id: number,
    username: string,

    display_name: string,

    description?: string,

    created: string,

    presence_type: string,

    // Whether the user is banned/terminated.
    is_terminated: boolean,

    // The user's verified badge status.
    has_verified_badge: boolean,
}

export enum TreatmentType {
    FriendCarousel = "FriendCarousel",
    Carousel = "Carousel",
    SortlessGrid = "SortlessGrid",
}

export interface RecommendationsTopic {
    topic_id: number,

    /// Topic / Title
    topic?: string,

    /// Subtitle
    subtitle?: string,

    /// Type of topic
    treatment_type: TreatmentType,

    /// Array of recommendations
    recommendation_list: Array<Recommendation>,
}

export interface Recommendation {
    universe_id: number,
    root_place_id: number,
    name: string,
    description?: string,

    total_up_votes: number,
    total_down_votes: number,
    player_count: number,
}

export enum AvatarType {
    MorphToR6 = "MorphToR6",
    MorphToR15 = "MorphToR15",
    PlayerChoice = "PlayerChoice"
}

export interface GameDetails {
    universe_id: number,
    root_place_id: number,

    /// Translated game's name
    name: string,

    /// Translated game's description
    description: string,

    /// Original game's name
    source_name: string,

    /// Original game's description
    source_description: string,

    creator: any,

    price?: number,

    allowed_gear_genres: string[],
    allowed_gear_categories: string[],
    is_genre_enforced: boolean,
    copying_allowed: boolean,

    playing: number,
    visits: number,
    max_players: number,
    created: string,
    updated: string,

    /// Avatar type. Possible values are MorphToR6, MorphToR15, and PlayerChoice
    universe_avatar_type: string,

    genre: String,
    is_all_genre: boolean,

    is_favorited_by_user: boolean,
    favorited_count: number,
}