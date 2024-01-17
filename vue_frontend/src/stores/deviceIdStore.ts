import { registerSelf } from "@/backend_calls/registration";
import { defineStore } from "pinia";

export const useDeviceIdStore = defineStore("device_id", {
    state: () => {
        return {
            device_id: "",
            device_id_valid: false,
        };
    },
    actions: {
        async registerSelf() {
            const response = await registerSelf("device_name");
            if (response.isOk()) {
                this.device_id_valid = true;
                this.device_id = response.value.client_id;
                console.log(response.value.client_id);
            }
            return response;
        },
    },
});
