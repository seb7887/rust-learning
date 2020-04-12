struct A; // Concrete type A
struct S(A); // Concrete type S
struct SGen<T>(T); // Generic type SGen

fn gen_spect_t(_s: SGen<A>) {}

fn gen_spect_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
  gen_spect_t(SGen(A));
  gen_spect_i32(SGen(6));

  generic(SGen('c'));
}
