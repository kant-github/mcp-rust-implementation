"use client";
import { FormEvent, JSX, useState } from "react";
import {
    Dialog,
    DialogTrigger,
    DialogContent,
    DialogHeader,
    DialogTitle,
} from "./ui/dialog";
import { Button } from "./ui/button";
import useCreateWallet from "@/hooks/api/useCreateWallet";
import { Input } from "./ui/input";

export default function CreateWalletDialog(): JSX.Element {
    const [open, setOpen] = useState(false);
    const [name, setName] = useState("");
    const { isPending, mutate } = useCreateWallet();

    function submit(e: FormEvent) {
        e.preventDefault();
        const trimmed = name.trim();
        if (!trimmed) return;
        mutate(trimmed, {
            onSuccess: () => {
                setName("");
                setOpen(false);
            },
        });
    }

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger render={<Button className="font-medium" />}>
                create wallet
            </DialogTrigger>

            <DialogContent>
                <DialogHeader>
                    <DialogTitle>Create wallet</DialogTitle>
                </DialogHeader>

                <form onSubmit={submit} className="flex flex-col gap-4">
                    <Input
                        autoFocus
                        placeholder="Treasury — main"
                        value={name}
                        onChange={(e) => setName(e.target.value)}
                    />
                    <Button type="submit" disabled={isPending || !name.trim()}>
                        {isPending ? "Creating…" : "Create"}
                    </Button>
                </form>
            </DialogContent>
        </Dialog>
    );
}