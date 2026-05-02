import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

class ClickingState {
    private _is_clicking = $state(false);
    readonly is_clicking = $derived(this._is_clicking);

    constructor() {
        listen("clicking-update", (e) => {
            this._is_clicking = e.payload as boolean
        })
    }

    async toggle() {
        invoke("toggle_clicking")
    }
}

export default new ClickingState()