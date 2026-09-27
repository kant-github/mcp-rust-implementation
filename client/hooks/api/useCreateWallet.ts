"use client";
import { api } from "@/lib/axios";
import { Wallet } from "@/types/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

export default function useCreateWallet() {
    const qc = useQueryClient();
    return useMutation({
        mutationFn: async (name: string) => {
            const res = await api.post<Wallet>("/v1/wallet", { name });
            return res.data;
        },
        onSuccess: () => {
            qc.invalidateQueries({ queryKey: ["wallets"] });
        },
    });
}