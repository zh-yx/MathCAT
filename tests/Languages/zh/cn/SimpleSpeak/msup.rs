/// Tests for superscripts
///   simple superscripts
///   complex/nested superscripts
use crate::common::*;

#[test]
fn squared() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2</mn> </msup>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "x 的平方");
}

#[test]
fn cubed() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>3</mn> </msup>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "x 的立方");
}

#[test]
    fn ordinal_power() {
        let expr = "<math>
                        <msup> <mi>x</mi> <mn>4</mn> </msup>
                    </math>";
        test("zh-cn", "SimpleSpeak", expr, "x 的 4 次方");
    }

#[test]
fn simple_mi_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mi>n</mi> </msup>
                </math>";
  test("zh-cn", "SimpleSpeak", expr, "x 的 n 次方");
}

#[test]
fn zero_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>0</mn> </msup>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "x 的 0 次方");
}


#[test]
fn decimal_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2.0</mn> </msup>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "x 的 2.0 次方");
}

#[test]
fn non_simple_power() {
    let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <mi>y</mi><mo>+</mo><mn>2</mn></mrow>
      </msup>
      </mrow>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "3 的 y 加 2 次方,");
}

#[test]
fn negative_power() {
    let expr = "<math>
                    <msup>
                        <mi>x</mi>
                        <mrow> <mo>-</mo> <mn>2</mn> </mrow>
                    </msup>
                </math>";
    test("zh-cn", "SimpleSpeak", expr, "x 的 负 2 次方");
}

#[test]
fn simple_fraction_power() {
  let expr = "<math>
                  <msup>
                      <mi>x</mi> 
                      <mfrac><mn>1</mn><mn>3</mn></mfrac>
                  </msup>
              </math>";
  test("zh-cn", "SimpleSpeak", expr, "x 的 3 分之 1, 次方,");
}

#[test]
fn nested_squared_power_with_coef() {
    let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <mn>2</mn>
        <msup>
          <mi>x</mi>
          <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test("zh-cn", "SimpleSpeak", expr, "3 的 2 x 的平方 次方,");
}

#[test]
fn nested_squared_power_with_neg_coef() {
    let expr = "<math>
    <mrow>
    <msup>
      <mn>3</mn>
      <mrow>
      <mo>-</mo>
      <mn>2</mn>
      <msup>
        <mi>x</mi>
        <mn>2</mn>
      </msup>
      </mrow>
    </msup>
    </mrow>
  </math>";
  test("zh-cn", "SimpleSpeak", expr, "3 的 负 2 x 的平方 次方,");
}


#[test]
fn nested_cubed_power() {
    let expr = "<math>
      <msup>
      <mi>y</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mn>3</mn>
      </msup>
    </msup>
  </math>";
  test("zh-cn", "SimpleSpeak", expr, "y 的 5 分之 4, 的立方 次方,");
}

#[test]
fn nested_cubed_power_with_neg_base() {
    let expr = "<math>
      <msup>
      <mi>y</mi> 
        <mrow>
            <mo>-</mo>
            <msup>
                <mfrac><mn>4</mn><mn>5</mn></mfrac>
                <mn>3</mn>
            </msup>
        </mrow>
    </msup>
    </math>";
  test("zh-cn", "SimpleSpeak", expr, "y 的 负 5 分之 4, 的立方 次方,");
}

#[test]
fn nested_number_times_squared() {
    let expr = "<math>
        <mrow>
        <msup>
          <mi>e</mi>
          <mrow>
          <mfrac>
            <mn>1</mn>
            <mn>2</mn>
            </mfrac>
            <msup>
            <mi>x</mi>
            <mn>2</mn>
            </msup>
          </mrow>
        </msup>
        </mrow>
        </math>";
  test("zh-cn", "SimpleSpeak", expr, "e 的 2 分之 1, x 的平方 次方,");
}

#[test]
fn nested_negative_number_times_squared() {
  let expr = "<math>
    <mrow>
    <msup>
      <mi>e</mi>
      <mrow>
      <mo>&#x2212;</mo><mfrac>
        <mn>1</mn>
        <mn>2</mn>
      </mfrac>
      <msup>
        <mi>x</mi>
        <mn>2</mn>
      </msup>
      </mrow>
    </msup>
    </mrow>
    </math>";
  test("zh-cn", "SimpleSpeak", expr, "e 的 负 2 分之 1, x 的平方 次方,");
}

#[test]
fn nested_expr_to_tenth() {
    let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <msup>
          <mn>3</mn>
          <mrow>
          <mn>10</mn></mrow>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test("zh-cn", "SimpleSpeak", expr, "3 的 3 的 10 次方 次方,");
}

#[test]
fn nested_non_simple_squared_exp() {
    let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <msup>
          <mrow>
          <mrow><mo>(</mo>
            <mrow>
            <mi>x</mi><mo>+</mo><mn>1</mn></mrow>
          <mo>)</mo></mrow></mrow>
          <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test("zh-cn", "SimpleSpeak", expr, "3 的 开括号 x 加 1 关括号 的平方 次方,");
}

#[test]
fn nested_simple_power() {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mi>n</mi>
      </msup>
    </msup>
  </math>";
  test("zh-cn", "SimpleSpeak", expr, "t 的 5 分之 4, 的 n 次方 次方,");
}

#[test]
fn nested_end_exponent_power() {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow>
      </msup>
    </msup>
  </math>";
  test("zh-cn", "SimpleSpeak", expr, "t 的 5 分之 4, 的 n 加 1 次方; 次方;");
  test_prefs("zh-cn", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
  "t 的 5 分之 4, 的 n 加 1 次方; 次方;");
}

#[test]
fn nested_end_exponent_neg_power() {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mrow><mo>-</mo><mn>3</mn></mrow>
      </msup>
    </msup>
  </math>";
  test("zh-cn", "SimpleSpeak", expr, "t 的 5 分之 4, 的 负 3 次方, 次方;");
}

#[test]
fn nested_complex_power() {
    let expr = "<math>
      <mrow>
      <msup>
        <mi>e</mi>
        <mrow>
        <mo>&#x2212;</mo><mfrac>
          <mn>1</mn>
          <mn>2</mn>
        </mfrac>
        <msup>
          <mrow>
          <mrow><mo>(</mo>
            <mrow>
            <mfrac>
              <mrow>
              <mi>x</mi><mo>&#x2212;</mo><mi>&#x03BC;</mi></mrow>
              <mi>&#x03C3;</mi>
            </mfrac>
            </mrow>
          <mo>)</mo></mrow></mrow>
          <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test("zh-cn", "SimpleSpeak", expr, "e 的 负 2 分之 1, 乘以; 开括号, 分式, x 减 mu, 比上 sigma, 结束分式; 关括号 的平方 次方,");
}

#[test]
fn default_power() {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <mfrac>
          <mrow><mi>b</mi><mo>+</mo><mn>1</mn></mrow>
          <mn>3</mn>
      </mfrac>
    </msup>
  </math>";
  test("zh-cn", "SimpleSpeak", expr, "t 的 分式, b 加 1, 比上 3, 结束分式; 次方,");
}
