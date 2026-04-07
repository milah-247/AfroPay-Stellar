interface Props { asset: string; balance: string; }

export default function BalanceCard({ asset, balance }: Props) {
  return (
    <div className="bg-gray-800 rounded-xl p-4 text-center">
      <p className="text-gray-400 text-sm">{asset}</p>
      <p className="text-xl font-bold mt-1">{parseFloat(balance).toFixed(2)}</p>
    </div>
  );
}
