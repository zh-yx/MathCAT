use crate::common::*;

#[test]
fn msqrt_simple() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "平方根 x,");
}

#[test]
fn neg_without_root() {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "负 x 减 y");
}

#[test]
fn msqrt() {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "开 x 加 y 的平方根,");
}

#[test]
fn mroot_as_square_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "平方根 x,");
}

#[test]
fn cube_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "立方 根 x,");
}

#[test]
fn ordinal_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "9 次方 根 x,");
}

#[test]
fn ordinal_root_2() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9.1</mn> </mroot>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "9.1 次方 根 x,");
}

#[test]
fn simple_mi_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "n 次方 根 x,");
}


#[test]
fn simple_fraction_power() {
    let expr = "<math>
                    <mroot>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </mroot>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "3 分之 1, 次方 根 x,");
}
