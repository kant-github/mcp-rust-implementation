import CreateWalletDialog from "@/components/CreateWalletDialog";
import WalletList from "@/components/WalletList";

export default function Home() {
  return (
    <main className="mx-auto max-w-2xl px-6 py-16">
      <div className="mb-8 flex items-center justify-between">
        <h1 className="text-2xl font-semibold">Wallets</h1>
        <CreateWalletDialog />
      </div>
      <WalletList />
    </main>
  );
}