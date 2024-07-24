import { invoke, type InvokeArgs } from "@tauri-apps/api/tauri";
import { readable, writable } from "svelte/store";
import { Store } from "tauri-plugin-store-api";

import {
    ThumbnailSize,
    ThumbnailType,
    type ClientInfo,
    type FriendUserInformation,
    type GameDetails,
    type GameMedia,
    type GameServer,
    type PlaceDetails,
    type RecommendationsTopic,
    type UserPresence
} from "$lib/typings";
import { STORE_PATH } from "./constants";

console.log("RAWWRH 👹👹👹👹")


export const robloxApi = {
    _invoke<T>(method: string, args?: InvokeArgs) {
        return invoke<T>(`plugin:roblox-api|${method}`, args);
    },

    auth(roblosecurity: string) {
        return this._invoke<void>("auth", { roblosecurity });
    },

    isAuthed() {
        return this._invoke<boolean>("is_authed");
    },

    updatePresence() {
        return this._invoke<void>("presence");
    },

    getMe() {
        return this._invoke<ClientInfo>("get_me");
    },

    getFriendsList() {
        return this._invoke<FriendUserInformation[]>("friends_list");
    },

    getPresences(userIds: number[]) {
        return this._invoke<UserPresence[]>("get_presences", {
            userIds,
        });
    },

    getRecommendations() {
        return this._invoke<RecommendationsTopic[]>("recommendations");
    },

    getPlaceDetails(placeId: number) {
        return this._invoke<PlaceDetails>("place_details", {
            placeId,
        });
    },

    getGameMedia(universeId: number) {
        return this._invoke<GameMedia[]>("game_media", {
            universeId,
        });
    },

    getGameDetails(universeId: number) {
        return this._invoke<GameDetails>("game_details", {
            universeId,
        });
    },

    getGameServers(placeId: number, serversType: "Public" | "Friends", cursor?: string) {
        return this._invoke<[GameServer[], string | null]>("game_servers", {
            placeId,
            serversType,
            cursor
        });
    },

    getAvatarsHeadshots(avatarIds: number[]) {
        return this._invoke<string[]>("get_headshots", {
            avatarIds,
        });
    },

    getThumbnailsUrls(
        ids: number[],
        thumbnailSize: ThumbnailSize,
        thumbnailType: ThumbnailType
    ) {
        return this._invoke<any[]>("thumbnail_url_bulk", {
            ids,
            thumbnailSize,
            thumbnailType
        })
    },

    getTokensThumbnailsUrls(
        tokens: string[],
        thumbnailSize: ThumbnailSize,
        thumbnailType: ThumbnailType
    ) {
        return this._invoke<string[]>("token_thumbnail_url_bulk", {
            tokens,
            thumbnailSize,
            thumbnailType
        })
    },


    playPlace(placeId: number) {
        return this._invoke<GameDetails>("open_place", {
            placeId,
        });
    },

    playServer(placeId: number, gameId: string) {
        return this._invoke<GameDetails>("open_server", {
            placeId,
            gameId,
        });
    },
}
