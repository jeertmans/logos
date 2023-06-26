use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use logos_derive::Logos;

static NUMBERS: &str = r"
86367226821763613943501353474164692147606940216546320653290381945746660557946985
00035253343071698244771775677428528197428012810782889087683316785017457757692219
08261507569533240712786062950522001196744254749027359476108419938721481823948019
47689574048245955980503163935717740581159866238466956566598280301739645350221110
32523628140840729289664849456823321434625329509036669166271371045579575445034085
96388021956382029206436244619163276532334510963355342979383844821837232636817666
60093423797021224966234208373526189366074767031190468177900530843595897861681676
39373243928923863460884944812114849823244042638531710296945184862886300155250699
58179444613213275161325108970092611363404703743321606785500336468355176383914343
11892539959676055361648972535923031050370907059847668292294596747599500693904553
";

#[derive(Debug, Clone, Copy, PartialEq, Logos)]
pub enum TokenAlternate {
    #[regex(r"90|91|92|93|94|95|96|97|98|99")]
    Ninetees,
}

#[derive(Debug, Clone, Copy, PartialEq, Logos)]
pub enum TokenRange {
    #[regex(r"[90-99]")]
    Ninetees,
}

fn count_ok_alternate(s: &str) -> usize {
    use logos::Logos;

    TokenAlternate::lexer(s).filter_map(|res| res.ok()).count()
}

fn count_ok_range(s: &str) -> usize {
    use logos::Logos;

    TokenRange::lexer(s).filter_map(|res| res.ok()).count()
}

fn bench_count_ok(c: &mut Criterion) {

    let mut group = c.benchmark_group("count_ok");
    group.throughput(Throughput::Bytes(NUMBERS.len() as u64));
    group.bench_with_input("alternate", &NUMBERS, |b, &s| b.iter(|| count_ok_alternate(s)));
    group.bench_with_input("range", &NUMBERS, |b, &s| b.iter(|| count_ok_range(s)));
}

criterion_group!(benches, bench_count_ok);
criterion_main!(benches);
