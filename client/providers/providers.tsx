"use client"
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useState } from "react";
import { JSX } from "react/jsx-runtime";

export default function Providers({ children }: { children: React.ReactNode }): JSX.Element {
    const [client] = useState(() => new QueryClient());
    return (
        <QueryClientProvider client={client}>
            {children}
        </QueryClientProvider>
    )
}