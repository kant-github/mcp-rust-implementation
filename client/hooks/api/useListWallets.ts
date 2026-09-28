import { api } from "@/lib/axios";
import { Wallet } from "@/types/types";
import { useQuery } from "@tanstack/react-query";

export default function useListWallets() {
    return useQuery({
        queryKey: ["wallets"],
        queryFn: async () => {
            const res = await api.get<Wallet[]>("/v1/wallet");
            return res.data;
        },
    })
}