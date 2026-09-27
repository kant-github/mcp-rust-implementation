export interface Wallet {
    id: string;
    name: string;
    address: string;
    status: "generating" | "ready" | "failed";
    createdAt: string;
}