import { invoke, InvokeArgs } from "@tauri-apps/api/tauri";
import { defineStore } from "pinia";
import {
    FriendUserInformation,
    GameDetails,
    PlaceDetails,
    RecommendationsTopic,
    UserPresence,
} from "./typings";

export interface ClientInfo {
    userId: number;
    username: string;
    display_name: string;
    robux: number;
}

export interface RobloxApi {
    cookie: string;
    clientInfo: ClientInfo | undefined;

    isLoggedIn(): boolean;
}

export const useRobloxApi = defineStore("robloxApi", {
    state: () => ({
        cookie: "",
        clientInfo: undefined as ClientInfo | undefined,
    }),

    getters: {
        isLoggedIn(): boolean {
            return this.cookie !== "";
        },
    },

    actions: {
        loadCookie(cookie: string) {
            this.cookie = cookie;
        },
        loadClientInfo(clientInfo: ClientInfo) {
            this.clientInfo = clientInfo;
        },

        _invoke<T>(method: string, args?: InvokeArgs) {
            return invoke<T>(`plugin:roblox-api|${method}`, args);
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

        getGameDetails(universeId: number) {
            return this._invoke<GameDetails>("game_details", {
                universeId,
            });
        },

        getAvatarsHeadshots(avatarIds: number[]) {
            return this._invoke<string[]>("get_headshots", {
                avatarIds,
            });
        },

        getGamesIcons(universeIds: number[]) {
            return this._invoke<string[]>("get_icons", {
                universeIds,
            });
        },

        getGamesThumbnails(placeIds: number[]) {
            return this._invoke<string[]>("get_head_thumbnails", {
                placeIds,
            });
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
    },
});
