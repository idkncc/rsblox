import {defineStore} from "pinia";

export interface ClientInfo {
    userId: number,
    username: string,
    display_name: string,
    robux: number
}

export interface RobloxApi {
    cookie: string,
    clientInfo: ClientInfo | undefined,

    isLoggedIn(): boolean
}

export const useRobloxApi = defineStore("robloxApi", {
    state: () => ({
        cookie: "",
        clientInfo: undefined as ClientInfo | undefined
    }),

    getters: {
        isLoggedIn(): boolean {
            return this.cookie !== ""
        }
    },

    actions: {
        loadCookie(cookie: string) {
            this.cookie = cookie
        },
        loadClientInfo(clientInfo: ClientInfo) {
            this.clientInfo = clientInfo
        }
    },
})
