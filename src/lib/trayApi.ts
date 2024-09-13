import { invoke, type InvokeArgs } from "@tauri-apps/api/tauri";

import {
    type TrayGame
} from "$lib/typings";

// Tray api
export const trayApi = {
    _invoke<T>(method: string, args?: InvokeArgs) {
        return invoke<T>(`plugin:tray-api|${method}`, args);
    },

    setGames(games: TrayGame[]) {
        return this._invoke<void>("tray_update", { games });
    }
}
