import numpy as np
import scipy.stats as st
import toml

DATA_SIZES = [10, 100, 1_000, 10_000, 100_000]
CONFIDENCE_LEVELS = [0.7, 0.9, 0.95, 0.99]
SEEDS = [0xfeedcafe, 0xdeadbeef, 0xbabeface]

if __name__ == "__main__":
    print(f"Generating: {DATA_SIZES} {CONFIDENCE_LEVELS} with {len(SEEDS)} cases each")
    # set parameters
    for size in DATA_SIZES:
        for level in CONFIDENCE_LEVELS:
            for (i, seed) in enumerate(SEEDS):
                case_no = i + 1
                
                # set seed
                np.random.seed(seed)

                # generate random data
                # data = [ np.random.randint(0, 100) for _ in range(size) ]
                # data = np.random.lognormal(mean=10.1, sigma=10, size=size)
                # negative binomial: number of attempts until n successes, with probability of success p
                data = np.random.negative_binomial(n=10, p=0.1, size=size)
                
                # compute the solutions
                (ci_lo, ci_hi) = st.t.interval(confidence=level, df=size-1,
                        loc=np.mean(data),
                        scale=st.sem(data))
                
                # write to file
                filename = f"tests/cases/case_size_{size}_lvl_{level}_no_{case_no}.toml"
                data_dict = {
                    "size": size,
                    "level": level,
                    "case": case_no,
                    "ci_low": ci_lo.item(),
                    "ci_high": ci_hi.item(),
                    "data": data.tolist(),
                }
                
                print(f">-> >-> >-> >-> >-> >-> {size} {level} {seed:08x} <-< <-< <-< <-< <-< <-<")
                print(filename)
                
                with open(filename, "w") as f:
                    toml.dump(data_dict, f)
