use aoc_runner_derive::*;
use std::fmt::{Display, Formatter};
use std::error::Error;

use std::ops::{Add, Mul, Index, IndexMut};

use boxed_array_ringbuffer::RingBuffer;


const PART1COUNT : usize = 80;
const PART2COUNT : usize = 256;


#[derive(Clone)]
pub struct Fishtank {
    adults : RingBuffer<usize,7>,
    babies : RingBuffer<usize,2>,
}

impl Fishtank {
    fn count(&self) -> usize {
        self.adults.iter().chain(self.babies.iter()).sum()
    }
    fn progress(self) -> Self {
        let births = self.adults[0];
        let (babies, grown_ups) = self.babies.push_pop(births);
        let new_adults = self.adults[0] + grown_ups;
        let (adults, _) = self.adults.push_pop(new_adults);
        Self{adults,babies}
    }
}

fn let_fishtank_grow_for_days(fishtank : Fishtank, days : usize) -> Fishtank {
    (0..days).fold(fishtank, |fishtank,_| fishtank.progress())
}

#[derive(Debug)]
pub struct ParseInputError;
impl Display for ParseInputError {
    fn fmt(&self, f : &mut Formatter) -> Result<(),std::fmt::Error> {
        write!(f,"Failed to parse input")
    }
}
impl Error for ParseInputError {}
#[aoc_generator(day6)]
pub fn input_generator(input : &str) -> Result<Fishtank, ParseInputError> {
    let adults = input.split(",").map(str::parse).try_fold(RingBuffer::new_copy(0), |mut buffer, age| {
        match age {
            Ok(age) => {
                if let Some(entry) = buffer.get_mut(age) {
                    *entry +=1;
                    Ok(buffer)
                }
                else {
                    Err(ParseInputError)
                }
            }
            Err(_) => { Err(ParseInputError) }
        }
    })?;
    Ok(Fishtank {
        adults : adults,
        babies : RingBuffer::new_copy(0)
    })
}

#[aoc(day6,part1, fishtank)]
fn solve_day6_part1_fishtank(input : &Fishtank) -> usize {
    let_fishtank_grow_for_days(input.clone(), PART1COUNT).count()
}

#[aoc(day6,part2, fishtank)]
fn solve_day6_part2_fishtank(input : &Fishtank) -> usize {
    let_fishtank_grow_for_days(input.clone(), PART2COUNT).count()
}

//-------------------------------------------------------
//Closed solution derivation:
//
//First we implement matrix multiplication, because we don't want third party dependencies here.

#[derive(Clone, Debug)]
struct Matrix<T, const COLUMNS : usize, const ROWS : usize> 
where T : Copy+Add<Output=T>+Mul<Output=T>+Default+std::iter::Sum
{
    storage : [[T; COLUMNS]; ROWS],
}

impl<T, const COLUMNS : usize, const ROWS : usize> Default for Matrix<T, COLUMNS, ROWS>
where T : Copy+Add<Output=T>+Mul<Output=T>+Default+std::iter::Sum
{
    fn default() -> Self { Self {storage : [[T::default(); COLUMNS]; ROWS]} }
}

impl<T, const COLUMNS : usize, const ROWS : usize> Index<usize> for Matrix<T, COLUMNS, ROWS> 
    where T : Copy+Add<Output=T>+Mul<Output=T>+Default+std::iter::Sum
{
    type Output = [T; COLUMNS];
    fn index(&self, index : usize) -> &Self::Output {
        &self.storage[index]
    }
}

impl<T, const COLUMNS : usize, const ROWS : usize> IndexMut<usize> for Matrix<T, COLUMNS, ROWS>
    where T: Copy+Add<Output=T>+Mul<Output=T>+Default+std::iter::Sum
{
    fn index_mut(&mut self, index : usize) -> &mut Self::Output {
        &mut self.storage[index]
    }
}

fn dot<T, const R1 : usize, const C1R2 : usize, const C2 : usize>(l : Matrix<T, C1R2, R1>, r : Matrix<T, C2, C1R2>) -> Matrix<T, C2, R1> 
where T : Copy+Add<Output=T>+Mul<Output=T>+Default+std::iter::Sum
{
    //the Matrix::default() call is ugly and wastes cycles, but afaict the only currently sane way to pull this
    //off... There'd be the https://docs.rs/array-init/ crate, or I could write my own unsafe code,
    //or use non-inline storage, but that all sounds ugly.
   (0..R1).flat_map(|row| (0..C2).map(move |column| (row, column))).fold(Matrix::default(), |mut result, (row, column)| {
       result[row][column] = l[row].iter().zip(0..C1R2).map(|(rv, cindex)| (rv, r[cindex][column])).map(|(&a,b)| a*b).sum();
       result
   })
}

impl<T, const R1 : usize, const C1R2 : usize, const C2 : usize> Mul<Matrix<T, C2, C1R2>> for Matrix<T, C1R2, R1> 
where T : Copy+Add<Output=T>+Mul<Output=T>+Default+std::iter::Sum
{
    type Output = Matrix<T, C2, R1>;
    fn mul(self, rhs : Matrix<T, C2, C1R2>) -> Self::Output {
        dot(self, rhs)
    }
}

impl<T, const CR : usize> std::iter::Product for Matrix<T, CR, CR>
where T : Copy+Add<Output=T>+Mul<Output=T>+Default+std::iter::Sum
{
    fn product<I>(iter: I) -> Self 
    where I: Iterator<Item = Self>
    {
        iter.reduce(|lhs, rhs| lhs * rhs).unwrap_or_default()
    }
}

fn diag_matrix<T, const RC : usize>(input : [T; RC]) -> Matrix<T, RC, RC>
where T : Copy+Add<Output=T>+Mul<Output=T>+Default+std::iter::Sum
{
    (0..RC).fold(Matrix::default(), |mut result, index| {
        result[index][index] = input[index];
        result
    })
}


//---------------------------------------------------------
//Now we implement a solution based on matrix multiplication.

fn get_fish_count_after_days_using_matrices(input : Matrix<usize, 1, 9>, days : usize) -> usize {
    let sum_up : Matrix<usize,9,1>
        = Matrix { storage : [[1,1,1,1,1,1,1,1,1]] };

    let progress_per_day : Matrix<usize, 9, 9> = Matrix { storage : [
        [0,1,0,0,0,0,0,0,0],
        [0,0,1,0,0,0,0,0,0],
        [0,0,0,1,0,0,0,0,0],
        [0,0,0,0,1,0,0,0,0],
        [0,0,0,0,0,1,0,0,0],
        [0,0,0,0,0,0,1,0,0], 
        [1,0,0,0,0,0,0,1,0],
        [0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0]
    ] };

    let total_progress = std::iter::repeat(progress_per_day).take(days).product();

    (sum_up * total_progress * input)[0][0]
}



#[aoc(day6, part1, Matrices)]
fn solve_part1_matrices(input : &Fishtank) -> usize {
    //this is ugly, but I don't have the concentration today to make this better...
    let adults = &input.adults;
    let babies = &input.babies;
    let initial_state : Matrix<usize, 1, 9> 
        = Matrix { storage : [[adults[0]], [adults[1]], [adults[2]], [adults[3]], [adults[4]], [adults[5]], [adults[6]], [babies[0]], [babies[1]]] };

    get_fish_count_after_days_using_matrices(initial_state, PART1COUNT)
}

#[aoc(day6, part2, Matrices)]
fn solve_part2_matrices(input : &Fishtank) -> usize {
    //this is ugly, but I don't have the concentration today to make this better...
    let adults = &input.adults;
    let babies = &input.babies;
    let initial_state : Matrix<usize, 1, 9> 
        = Matrix { storage : [[adults[0]], [adults[1]], [adults[2]], [adults[3]], [adults[4]], [adults[5]], [adults[6]], [babies[0]], [babies[1]]] };

    get_fish_count_after_days_using_matrices(initial_state, PART2COUNT)
}

//----------------------------------------------------------
//We now know that the above matrix indeed yields the correct solution. However multiplying 9x9
//matrices is slow. Sooo, let's find its Eigenvectors to get rid of those nasty matrix
//multiplications...
//First things first: it can already be seen by just looking at the matrix, that it's linearly
//independent. This means that it definitely can be diagonalized. However the Eigenvalues are
//likely complex, as the matrix is obviously not Hermitian.
//
//I'll now not implement an Eigenvalue solver here, as that's definitely beyond the scope of this
//exercise. I'll just document the steps of that calculation:
//
//First we need to find the characteristic polynomial. That's given by det(λ*E - M) = 0 where E is
//the Identity Matrix, M is the matrix we want to diagonalize, and λ is the variable we solve this
//characteristic polynomial for, each solution of which is an Eigenvalue.
//
//After typing the above equation into a calculator (in my case: my trusty TI-92), the
//characteristic polynomial is revealed: λ⁹ - λ² - 1 = 0
//
//Well, isn't that nice? A polynomial that doesn't yield at all to analytic solutions...
//We'll have to go with approximate solutions... If I were to implement a solver myself, I'd just
//go with the Weierstraß (also known as Durand-Kerner) method. However I'm not going to do that and
//rather just rely on a a website that implements the Weierstraß-Method to compute the roots:
//http://www.hvks.com/Numerical/websolver.php
//
//The roots are (after fixing symmetry around real axis):
//X1=(-0.9961306220554406+i0.41731183633579305)
//X2=(-0.9961306220554406-i0.41731183633579305)
//X3=(-0.37921398065481077+i0.892877546086168)
//X4=(-0.37921398065481077-i0.892877546086168)
//X5=(0.7340778984637529+i0.742065121962188)
//X6=(0.7340778984637529-i0.742065121962188)
//X7=(0.09575446900611988-i0.8701987186721044)
//X8=(0.09575446900611988+i0.8701987186721044)
//X9=(1.091024470480757)
//
//Now we need the Eigenvector for each of them. Those are defined as those vectors, that are just
//scaled by multiplying them with the matrix, with their scaling being the Eigenvalue.
//
//In other words, we have to solve the coupled linear equations M.v = λ*v for each value of λ.
//We need an additional constraint for that system of equations to be fully defined, and we'll pick
//that the Eigenvectors need to be normalized.
//
//I'm honestly too lazy to do that by hand. Wolfram Alpha spits out nice eigenvectors, but
//precision is lacking. I'll rather use LAPACK and GNU Maxima for that part...
//Soo, dgeev() is the way to go...
//
//In any case, I'll from now on use the output of GNU Maxmia and copy&paste stuff as needed, because
//frankly, the closed form is frightening.
//
//We also need a way to deal with complex numbers... I could drag in a third-party crate, but the
//only operations we need are addition, multiplication and exponents.
//
//I'll do a minimal (and slow) implementation below...

//Returns an array because I don't want to implement matrix iterators...
fn get_progress_matrix_eigenvalues() -> [Complex; 9] {
    [
        Complex::new(-0.9961306220554396,  0.4173118363357923), 
        Complex::new(-0.9961306220554396, -0.4173118363357923),
        Complex::new(1.091024470480757, 0.0),
        Complex::new(0.7340778984637535,  0.7420651219621887),
        Complex::new(0.7340778984637535, -0.7420651219621887),
        Complex::new(-0.3792139806548112,  0.8928775460861691),
        Complex::new(-0.3792139806548112, -0.8928775460861691),
        Complex::new(0.09575446900611977,  0.8701987186721045),
        Complex::new(0.09575446900611977, -0.8701987186721045)
    ]
}

fn get_progress_matrix_eigenvectors() -> Matrix<Complex,9,9> {
    Matrix { storage : { [
        [
            Complex::new(0.19899984657465, -0.1896211561334271),
            Complex::new(0.19899984657465, 0.1896211561334271),
            Complex::new(0.2664863857010273, 0.0),
            Complex::new(-0.009810288227828924, -0.3020749692844217),
            Complex::new(-0.009810288227828924, 0.3020749692844217),
            Complex::new(-0.2444548828945468, -0.2533423718859127),
            Complex::new(-0.2444548828945468, 0.2533423718859127),
            Complex::new(-0.3792659279170416, 0.08448995460471606),
            Complex::new(-0.3792659279170416, -0.08448995460471606)
        ],
        [
            Complex::new(-0.1190986880831869, 0.2719324316186708),
            Complex::new(-0.1190986880831869, -0.2719324316186708),
            Complex::new(0.290743167849794, 0.0),
            Complex::new(0.2169577831581597, -0.2290264313610791),
            Complex::new(0.2169577831581597, 0.2290264313610791),
            Complex::new(0.3189044245620895, -0.1221973066562757),
            Complex::new(0.3189044245620895, 0.1221973066562757),
            Complex::new(-0.1098394577774974, -0.32194643376987), 
            Complex::new(-0.1098394577774974, 0.32194643376987)
        ], 
        [
            Complex::new(0.005157227848246945, -0.3205815144945338),
            Complex::new(0.005157227848246945, 0.3205815144945338),
            Complex::new(0.3172079107492197, 0.0),
            Complex::new(0.32921644023662, -0.007126437606288196),
            Complex::new(0.32921644023662, 0.007126437606288196),
            Complex::new(-0.0118257849810276, 0.331081527121443),
            Complex::new(-0.0118257849810276, -0.331081527121443),
            Complex::new(0.2696397551921902, -0.1264099652316646),
            Complex::new(0.2696397551921902, 0.1264099652316646)
        ],
        [
            Complex::new(0.1286451879244679, 0.321493235676669),
            Complex::new(0.1286451879244679, -0.321493235676669),
            Complex::new(0.3460815928574744, 0.0),
            Complex::new(0.2469587933800816, 0.2390686775345879),
            Complex::new(0.2469587933800816, -0.2390686775345879),
            Complex::new(-0.2911307584936315, -0.1361097216953978),
            Complex::new(-0.2911307584936315, 0.1361097216953978),
            Complex::new(0.1358210013533483, 0.2225358503734635),
            Complex::new(0.1358210013533483, -0.2225358503734635)
        ],
        [
            Complex::new(-0.2623103436214063, -0.2665640972326937),
            Complex::new(-0.2623103436214063, 0.2665640972326937),
            Complex::new(0.3775834865904633, 0.0),
            Complex::new(0.003882464699551169, 0.3587545395223241),
            Complex::new(0.003882464699551169, -0.3587545395223241),
            Complex::new(0.2319301681252825, -0.2083294078640683),
            Complex::new(0.2319301681252825, 0.2083294078640683),
            Complex::new(-0.1806449439891257, 0.1395000635337823),
            Complex::new(-0.1806449439891257, -0.1395000635337823)
        ],
        [
            Complex::new(0.3725357186805364, 0.1560674488075285),
            Complex::new(0.3725357186805364, -0.1560674488075285),
            Complex::new(0.4119528235196378, 0.0),
            Complex::new(-0.2633691995976155, 0.2662348200776662),
            Complex::new(-0.2633691995976155, -0.2662348200776662),
            Complex::new(0.098061488182526, 0.2860866634226474),
            Complex::new(0.098061488182526, -0.2860866634226474),
            Complex::new(-0.1386903372320934, -0.1438392442839339),
            Complex::new(-0.1386903372320934, 0.1438392442839339)
        ],
        [
            Complex::new(-0.4362230308412252,0.0),
            Complex::new(-0.4362230308412252, 0.0),
            Complex::new(0.4494506111435653, 0.0),
            Complex::new(-0.3908970827922131, 0.0),
            Complex::new(-0.3908970827922131, 0.0),
            Complex::new(-0.2926266452874237, -0.02093116151478359),
            Complex::new(-0.2926266452874237, 0.02093116151478359),
            Complex::new(0.1118885064727042, -0.1344614042102196),
            Complex::new(0.1118885064727042, 0.1344614042102196)
        ],
        [
            Complex::new(0.2355352724921288, 0.007580122081110474),
            Complex::new(0.2355352724921288, -0.007580122081110474),
            Complex::new(0.2238752293291331, 0.0),
            Complex::new(-0.2771386208238903, 0.01200387786755439),
            Complex::new(-0.2771386208238903, -0.01200387786755439),
            Complex::new(0.3741119620297071, 0.0),
            Complex::new(0.3741119620297071, 0.0),
            Complex::new(0.5069878940968082, 0.0),
            Complex::new(0.5069878940968082, 0.0)
        ],
        [
            Complex::new(-0.2377871721688999, 0.09074086536162872),
            Complex::new(-0.2377871721688999, -0.09074086536162872),
            Complex::new(0.2442533535325755, 0.0),
            Complex::new(-0.2123489954513499, -0.1968431230236828),
            Complex::new(-0.2123489954513499, 0.1968431230236828),
            Complex::new(-0.1418684863318672, 0.3340361706185666),
            Complex::new(-0.1418684863318672, -0.3340361706185666),
            Complex::new(0.04854635659177065, 0.4411802158253109),
            Complex::new(0.04854635659177065, -0.4411802158253109)
        ]
    ]}}
}

fn get_progress_matrix_inverse_eigenvectors() -> Matrix<Complex,9,9> {
    Matrix { storage : { [
        [
            Complex::new(0.3486989487546765, 0.2989422454187725),
            Complex::new(-0.1908373790664648, -0.380051505431422),
            Complex::new(0.02700469819481069, 0.3928409356770213),
            Complex::new(0.1174845534168364, -0.3451487519178137),
            Complex::new(-0.2238159629573871, 0.2527255922474708),
            Complex::new(0.281557071404708, -0.1357537764145929),
            Complex::new(-0.2890191543835333, 0.01520148264425049),
            Complex::new(0.2522618184392547, 0.09042023009009366),
            Complex::new(-0.1830826837435938, -0.1674708088987285)
        ],
        [
            Complex::new(0.3486989487546764, -0.2989422454187725),
            Complex::new(-0.1908373790664648, 0.380051505431422),
            Complex::new(0.02700469819481068, -0.3928409356770213),
            Complex::new(0.1174845534168364, 0.3451487519178137),
            Complex::new(-0.2238159629573871, -0.2527255922474708),
            Complex::new(0.281557071404708, 0.1357537764145929),
            Complex::new(-0.2890191543835333, -0.01520148264425048),
            Complex::new(0.2522618184392547, -0.0904202300900937),
            Complex::new(-0.1830826837435938, 0.1674708088987285)
        ],
        [
            Complex::new(0.4742181559616455,0.0),
            Complex::new(0.434654005287969, 0.0),
            Complex::new(0.3983907025444076, 0.0),
            Complex::new(0.3651528570838175, 0.0),
            Complex::new(0.3346880541761948, 0.0),
            Complex::new(0.3067649381216127, 0.0),
            Complex::new(0.2811714552895747, 0.0),
            Complex::new(0.2577132437420745, 0.0),
            Complex::new(0.2362121572108401,0.0)
        ],
        [
            Complex::new(-0.06569063068384375, 0.4095251163189331),
            Complex::new(0.2346626794473929, 0.3206609638669744),
            Complex::new(0.3765032858042049, 0.05622156347968291),
            Complex::new(0.2919630559798539, -0.2185517880325913),
            Complex::new(0.04785909443822407, -0.3461027137795469),
            Complex::new(-0.2034806240242746, -0.2657849256929581),
            Complex::new(-0.3181188512993283, -0.04048619576648209),
            Complex::new(-0.2419090508777247, 0.1893887200088734),
            Complex::new(-0.03399749420961479, 0.2923628066518291)
        ],
        [
            Complex::new(-0.06569063068384372, -0.4095251163189331),
            Complex::new(0.2346626794473929, -0.3206609638669743),
            Complex::new(0.3765032858042049, -0.0562215634796829),
            Complex::new(0.2919630559798539, 0.2185517880325913),
            Complex::new(0.04785909443822408, 0.3461027137795469),
            Complex::new(-0.2034806240242745, 0.2657849256929581),
            Complex::new(-0.3181188512993282, 0.0404861957664821),
            Complex::new(-0.2419090508777247, -0.1893887200088735),
            Complex::new(-0.0339974942096148, -0.2923628066518291)
        ],
        [
            Complex::new(-0.1566433899208904, 0.2940638465219954),
            Complex::new(0.3421390950970197, 0.03012670869530254),
            Complex::new(-0.109288840833082, -0.3367709716092889),
            Complex::new(-0.2754964271512365, 0.2394067793128841),
            Complex::new(0.3381739498974908, 0.1649231051921171),
            Complex::new(0.02020740668890759, -0.3873284556685744),
            Complex::new(-0.375650591965116, 0.1369107669482404),
            Complex::new(0.281282749603226, 0.3012554655200025),
            Complex::new(0.172488949725062, -0.3882872543576955)
        ],
        [
            Complex::new(-0.1566433899208905, -0.2940638465219954),
            Complex::new(0.3421390950970197, -0.03012670869530257),
            Complex::new(-0.109288840833082, 0.3367709716092888),
            Complex::new(-0.2754964271512365, -0.2394067793128841),
            Complex::new(0.3381739498974908, -0.1649231051921171),
            Complex::new(0.02020740668890758, 0.3873284556685744),
            Complex::new(-0.375650591965116, -0.1369107669482404),
            Complex::new(0.281282749603226, -0.3012554655200025),
            Complex::new(0.172488949725062, 0.3882872543576955)
        ],
        [
            Complex::new(-0.1728543990754797, -0.0951149567557127),
            Complex::new(-0.1295910569370325, 0.1843779734304451),
            Complex::new(0.1931546301457375, 0.1701754700451486),
            Complex::new(0.217352157878041, -0.1980492340229828),
            Complex::new(-0.1977124703087888, -0.2715287961454471),
            Complex::new(-0.3329996849882462, 0.190561372632885),
            Complex::new(0.1747618480311107, 0.4019012042245666),
            Complex::new(0.4781590532795363, -0.1482144009366888),
            Complex::new(-0.1085445157345817, -0.5614265629955691)
        ],
        [
            Complex::new(-0.1728543990754797, 0.09511495675571273),
            Complex::new(-0.1295910569370325, -0.1843779734304451),
            Complex::new(0.1931546301457375, -0.1701754700451486),
            Complex::new(0.2173521578780411, 0.1980492340229828),
            Complex::new(-0.1977124703087888, 0.2715287961454471),
            Complex::new(-0.3329996849882462, -0.1905613726328849),
            Complex::new(0.1747618480311107, -0.4019012042245666),
            Complex::new(0.4781590532795364, 0.1482144009366888),
            Complex::new(-0.1085445157345817, 0.561426562995569)
        ]
    ]}}
}

#[cfg(test)]
mod eigenvector_tests{
    use super::*;
    #[test]
    fn test_inverse() {
        let a = get_progress_matrix_inverse_eigenvectors();
        let b = get_progress_matrix_eigenvectors();
        let should_be_unit = a*b;
        for (row, column) in  (0..9).flat_map(|row| (0..9).map(move |column| (row, column))) {
            assert!(should_be_unit[row][column].imag.abs() < 1e-10);
            assert!((should_be_unit[row][column].real - if row == column {1.0} else { 0.0 }).abs() < 1e-10);
        }
    }
}


#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Complex {
    real : f64,
    imag : f64
}

impl Add for Complex{
    type Output = Complex;
    fn add(self, rhs : Self) -> Self::Output {
        Complex { real : self.real + rhs.real, imag : self.imag + rhs.imag }
    }
}

impl Mul for Complex{
    type Output = Complex;
    fn mul(self, rhs : Self) -> Self::Output {
        Complex { 
            real : self.real*rhs.real - self.imag*rhs.imag, 
            imag : self.real*rhs.imag + self.imag*rhs.real,
        }
    }
}

impl std::iter::Sum for Complex {
    fn sum<I>(iter: I) -> Self
        where I : Iterator<Item=Self>
    {
        iter.reduce(|lhs, rhs| lhs + rhs).unwrap_or_default()
    }
}

impl Complex {
    fn new(real : f64, imag : f64) -> Complex {
        Complex { real, imag }
    }
    fn abs(self) -> f64 {
        (self.real*self.real + self.imag*self.imag).sqrt()
    }
    fn powi(self, exponent : i32) -> Complex {
        //this is waaaay easier to pull of in polar coordinates.
        let abs = self.abs();
        let angle = self.imag.atan2(self.real);
        let powered_abs = abs.powi(exponent);
        let powered_angle = exponent as f64 * angle;
        let (s,c) = powered_angle.sin_cos();
        Complex { real : powered_abs * c, imag : powered_abs * s }
    }
}

#[cfg(test)]
mod complex_tests{
    use super::*;
    #[test]
    fn test_complex_mul() {
        let a = Complex::new(3.4,7.2);
        let b = Complex::new(0.3,4.6);
        assert_eq!(a*b, b*a);
        assert!(((a*b).real + 32.1).abs() < 1e-10 );
        assert!(((a*b).imag -17.8).abs() < 1e-10 );
    }

    #[test]
    fn test_complex_add() {
        let a = Complex::new(3.4,7.2);
        let b = Complex::new(0.3,4.6);
        assert_eq!(a+b, b+a);
        assert_eq!((a+b).real, 3.4+0.3);
        assert_eq!((a+b).imag, 7.2+4.6);
    }

    #[test]
    fn test_complex_powi() {
        let c = Complex::new(0.78,1.2);
        let d = c.powi(5);
        assert!((1.5422086368 - d.real).abs() < 1e-10);
        assert!((5.80392864 + d.imag).abs() < 1e-10);
    }
}
//------------------------------------------------------------
//Now we have all numbers ready, we can actually start implementing the closed form. Took long
//enough..
fn get_fish_count_after_days_using_diagonal_matrices(input : Matrix<Complex, 1, 9>, days : i32) -> Complex {
    let c1 = Complex::new(1.0,0.0);
    let sum_up : Matrix<Complex,9,1>
        = Matrix { storage : [[c1,c1,c1,c1,c1,c1,c1,c1,c1]] };

    let convert_to_diagonal_space = get_progress_matrix_inverse_eigenvectors();
    let convert_back_to_fish_space = get_progress_matrix_eigenvectors();

    let progress_per_day = get_progress_matrix_eigenvalues();

    let progress_after_days = progress_per_day.map(|c| c.powi(days));

    let progress_after_days = diag_matrix(progress_after_days);

    (sum_up * convert_back_to_fish_space * progress_after_days * convert_to_diagonal_space * input)[0][0]
}



#[aoc(day6, part1, ClosedForm)]
fn solve_part1_closed_form(input : &Fishtank) -> f64 {
    //this is ugly, but I don't have the concentration today to make this better...
    let adults = &input.adults;
    let babies = &input.babies;
    let initial_state : Matrix<Complex, 1, 9> 
        = Matrix { storage : [
            [Complex::new(adults[0] as f64, 0.0)],
            [Complex::new(adults[1] as f64, 0.0)],
            [Complex::new(adults[2] as f64, 0.0)],
            [Complex::new(adults[3] as f64, 0.0)],
            [Complex::new(adults[4] as f64, 0.0)],
            [Complex::new(adults[5] as f64, 0.0)],
            [Complex::new(adults[6] as f64, 0.0)],
            [Complex::new(babies[0] as f64, 0.0)],
            [Complex::new(babies[1] as f64, 0.0)]
        ] };

    let fish_count = get_fish_count_after_days_using_diagonal_matrices(initial_state, PART1COUNT as i32);
    fish_count.real.round()
}

#[aoc(day6, part2, ClosedForm)]
fn solve_part2_closed_form(input : &Fishtank) -> f64 {
    //this is ugly, but I don't have the concentration today to make this better...
    let adults = &input.adults;
    let babies = &input.babies;
    let initial_state : Matrix<Complex, 1, 9> 
        = Matrix { storage : [
            [Complex::new(adults[0] as f64, 0.0)],
            [Complex::new(adults[1] as f64, 0.0)],
            [Complex::new(adults[2] as f64, 0.0)],
            [Complex::new(adults[3] as f64, 0.0)],
            [Complex::new(adults[4] as f64, 0.0)],
            [Complex::new(adults[5] as f64, 0.0)],
            [Complex::new(adults[6] as f64, 0.0)],
            [Complex::new(babies[0] as f64, 0.0)],
            [Complex::new(babies[1] as f64, 0.0)]
        ] };


    let fish_count = get_fish_count_after_days_using_diagonal_matrices(initial_state, PART2COUNT as i32);
    fish_count.real.round()
}

#[cfg(test)]
mod day6_tests{
    use super::*;

    fn get_day6_string_testdata() -> &'static str { r#"3,4,3,1,2"# }

    #[test]
    fn test_day6_generator(){
        let fishtank = input_generator(get_day6_string_testdata()).unwrap();
        assert!(fishtank.adults.iter().eq([0,1,1,2,1,0,0].iter()));
        assert_eq!(fishtank.count(), 5);
    }

    #[test]
    fn test_day6_part1_fishtank() {
        let fishtank = input_generator(get_day6_string_testdata()).unwrap();
        let count = solve_day6_part1_fishtank(&fishtank);
        assert_eq!(count, 5934);
    }

    #[test]
    fn test_day6_part2_fishtank() {
        let fishtank = input_generator(get_day6_string_testdata()).unwrap();
        let count = solve_day6_part2_fishtank(&fishtank);
        assert_eq!(count,26984457539);
    }

    #[test]
    fn test_day6_part1_matrices() {
        let fishtank = input_generator(get_day6_string_testdata()).unwrap();
        let count = solve_part1_matrices(&fishtank);
        assert_eq!(count, 5934);
    }

    #[test]
    fn test_day6_part2_matrices() {
        let fishtank = input_generator(get_day6_string_testdata()).unwrap();
        let count = solve_part2_matrices(&fishtank);
        assert_eq!(count,26984457539);
    }
    
    #[test]
    fn test_day6_part1_closed_solution() {
        let fishtank = input_generator(get_day6_string_testdata()).unwrap();
        let count = solve_part1_closed_form(&fishtank);
        assert_eq!(count as usize, 5934);
    }

    #[test]
    fn test_day6_part2_closed_solution() {
        let fishtank = input_generator(get_day6_string_testdata()).unwrap();
        let count = solve_part2_closed_form(&fishtank);
        assert_eq!(count as usize,26984457539);
    }
}