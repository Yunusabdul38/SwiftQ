// TODO: /results/[roundId] page — Post-round results, prize breakdown, share prompt
export default async function ResultsPage({ params }: { params: Promise<{ roundId: string }> }) {
  const { roundId } = await params;
  return <div>Results for Round {roundId} Stub</div>;
}
