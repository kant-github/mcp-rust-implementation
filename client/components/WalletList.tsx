"use client";

import useListWallets from "@/hooks/api/useListWallets";
import { Wallet } from "@/types/types";


export default function WalletList() {
    const { data: wallets, isPending, error } = useListWallets();

    if (isPending) return <p className="text-sm text-neutral-500">Loading…</p>;
    if (error) return <p className="text-sm text-red-500">{error.message}</p>;
    if (!wallets?.length) return <p className="text-sm text-neutral-500">No wallets yet.</p>;

    return (
        <ul className="space-y-2">
            {wallets.map((w: Wallet) => (
                <li key={w.id} className="rounded-md border px-4 py-3">
                    <div className="flex items-center justify-between gap-3">
                        <p className="text-sm font-medium">{w.name}</p>
                        <span className="text-xs text-neutral-500">
                            {w.status}
                        </span>
                    </div>
                    <p className="font-mono text-xs text-neutral-500">{w.address}</p>
                    <p className="mt-1 text-xs text-neutral-400">
                        {new Date(w.created_at).toLocaleString()}
                    </p>
                </li>
            ))}
        </ul>
    );
}