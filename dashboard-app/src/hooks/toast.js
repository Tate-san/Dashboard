import { writable } from "svelte/store";

export const toasts = writable([]);

export const addToast = (toast) => {
    const id = Math.floor(Math.random() * 10000);

    const defaults = {
        id,
        message: "This is a toast",
        type: "success",
        icon: "fa-check",
        color: "text-green-400",
        dismissable: true,
        open: true,
        timeout: 3000,
    };

    switch(toast.type) {
        case "success":
            toast.icon = "fa-check";
            toast.color = "text-green-400"; 
            break;

        case "error":
            toast.icon = "fa-xmark";
            toast.color = "text-red-400"; 
            break;

        case "warning":
            toast.icon = "fa-circle-exclamation";
            toast.color = "text-orange-400"; 
            break;

        default:
            break;
    }

    const newToast = {
        ...defaults,
        ...toast
    };

    toasts.update((all) => [...all, newToast]);

    if (newToast.timeout) setTimeout(() => {
        toasts.update((all) => all.map(t => 
            t.id === id
            ? {...t, open: false}
            : t
        ));
    }, newToast.timeout);
};