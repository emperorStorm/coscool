export interface FormulaItem {
  label: string
  latex: string
}

export interface FormulaCategory {
  key: string
  name: string
  items: FormulaItem[]
}

export const formulaCategories: FormulaCategory[] = [
  {
    key: 'algebra',
    name: '代数',
    items: [
      { label: '因式乘积', latex: '$(x-1)(x+3)$' },
      { label: '勾股根式', latex: '$\\sqrt{a^2+b^2}$' },
      { label: '分式幂', latex: '$\\left(\\frac{a}{b}\\right)^n=\\frac{a^n}{b^n}$' },
      { label: '异分母加减', latex: '$\\frac{a}{b}\\pm\\frac{c}{d}=\\frac{ad\\pm bc}{bd}$' },
      { label: '双曲线标准式', latex: '$\\frac{x^2}{a^2}-\\frac{y^2}{b^2}=1$' },
      { label: '根式有理化', latex: '$\\frac{1}{\\sqrt a}=\\frac{\\sqrt a}{a},\\ a\\ge0$' },
      { label: 'n 次根式', latex: '$\\sqrt[n]{a^n}=(\\sqrt[n]{a})^n$' },
      { label: '一元二次求根', latex: '$x=\\frac{-b\\pm\\sqrt{b^2-4ac}}{2a}$' },
      { label: '点斜式', latex: '$y-y_1=k(x-x_1)$' },
      { label: '圆的参数方程', latex: '$$\\begin{cases}x=a+r\\cos\\theta\\\\y=b+r\\sin\\theta\\end{cases}$$' },
      {
        label: '三次单位根',
        latex:
          '$$\\begin{gathered}x^3-1=0\\\\\\omega=\\frac{-1+\\sqrt3 i}{2}\\\\x_1=1,\\ x_2=\\omega=\\frac{-1+\\sqrt3 i}{2}\\\\x_3=\\omega^2=\\frac{-1-\\sqrt3 i}{2}\\end{gathered}$$'
      },
      {
        label: '判别式分类',
        latex:
          '$$\\begin{gathered}ax^2+bx+c=0\\\\\\Delta=b^2-4ac\\\\\\begin{cases}\\Delta>0,\\ \\text{方程有两个不相等的实根}\\\\\\Delta=0,\\ \\text{方程有两个相等的实根}\\\\\\Delta<0,\\ \\text{方程没有实根}\\end{cases}\\end{gathered}$$'
      },
      {
        label: '韦达定理',
        latex:
          '$$\\begin{gathered}ax^2+bx+c=0\\\\\\Delta=b^2-4ac\\\\x_{1,2}=\\frac{-b\\pm\\sqrt{b^2-4ac}}{2a}\\\\x_1+x_2=-\\frac{b}{a}\\\\x_1x_2=\\frac{c}{a}\\end{gathered}$$'
      }
    ]
  },
  {
    key: 'geometry',
    name: '几何',
    items: [
      { label: '三角形', latex: '$\\triangle ABC$' },
      { label: '平行传递', latex: '$a \\parallel c,\\ b \\parallel c \\Rightarrow a \\parallel b$' },
      { label: '线面垂直推出面面垂直', latex: '$l \\perp \\beta,\\ l \\subset \\alpha \\Rightarrow \\alpha \\perp \\beta$' },
      {
        label: '同垂直同一平面',
        latex: '$$\\left.\\begin{array}{l}a \\perp \\alpha\\\\ b \\perp \\alpha\\end{array}\\right\\}\\Rightarrow a \\parallel b$$'
      },
      { label: '交线判定', latex: '$P \\in \\alpha,\\ P \\in \\beta,\\ \\alpha \\cap \\beta = l \\Rightarrow P \\in l$' },
      {
        label: '面面垂直性质',
        latex: '$\\alpha \\perp \\beta,\\ \\alpha \\cap \\beta = l,\\ a \\subset \\alpha,\\ a \\perp l \\Rightarrow a \\perp \\beta$'
      },
      {
        label: '面面平行判定',
        latex: '$$\\left.\\begin{array}{l}a \\subset \\beta,\\ b \\subset \\beta,\\ a \\cap b = P\\\\ a \\parallel \\alpha,\\ b \\parallel \\alpha\\end{array}\\right\\}\\Rightarrow \\beta \\parallel \\alpha$$'
      },
      { label: '弧 AB', latex: '$\\overset{\\frown}{AB}$' },
      { label: '角 ABC', latex: '$\\angle ABC$' },
      { label: '角度', latex: '$\\angle A = 60^\\circ$' },
      { label: '线段垂直', latex: '$AB \\perp CD$' },
      { label: '线段平行', latex: '$AB \\parallel CD$' },
      { label: '线段 AB', latex: '$\\overline{AB}$' },
      { label: '向量 AB', latex: '$\\vec{AB}$' },
      { label: '圆 O', latex: '$\\odot O$' },
      { label: '三角形相似', latex: '$\\triangle ABC \\sim \\triangle DEF$' },
      { label: '三角形全等', latex: '$\\triangle ABC \\cong \\triangle DEF$' },
      { label: '三角形面积', latex: '$S_{\\triangle ABC}=\\frac{1}{2}ah$' },
      { label: '余弦定理', latex: '$c^2=a^2+b^2-2ab\\cos C$' }
    ]
  },
  {
    key: 'inequality',
    name: '不等式',
    items: [
      { label: '传递性', latex: '$a>b,\\ b>c \\Rightarrow a>c$' },
      { label: '加法性质', latex: '$a>b,\\ c>d \\Rightarrow a+c>b+d$' },
      { label: '正数乘法性质', latex: '$a>b>0,\\ c>d>0 \\Rightarrow ac>bd$' },
      { label: '乘法变号', latex: '$$\\begin{gathered}a>b,\\ c>0 \\Rightarrow ac>bc\\\\a>b,\\ c<0 \\Rightarrow ac<bc\\end{gathered}$$' },
      { label: '绝对值差', latex: '$|a-b|\\ge |a|-|b|$' },
      { label: '绝对值范围', latex: '$-|a|\\le a\\le |a|$' },
      { label: '绝对值小于 b', latex: '$|a|\\le b \\Rightarrow -b\\le a\\le b$' },
      { label: '三角不等式', latex: '$|a+b|\\le |a|+|b|$' },
      {
        label: '幂与根式单调性',
        latex: '$$a>b>0,\\ n\\in N^*,\\ n>1\\Rightarrow a^n>b^n,\\ \\sqrt[n]{a}>\\sqrt[n]{b}$$'
      },
      { label: '柯西不等式', latex: '$\\left(\\sum_{k=1}^{n}a_kb_k\\right)^2\\le\\left(\\sum_{k=1}^{n}a_k^2\\right)\\left(\\sum_{k=1}^{n}b_k^2\\right)$' },
      {
        label: '算术几何平均',
        latex: '$$a,b\\in R^+\\Rightarrow \\frac{a+b}{2}\\ge\\sqrt{ab}\\quad(\\text{当且仅当 }a=b\\text{ 时取等号})$$'
      },
      {
        label: '平方平均',
        latex: '$$a,b\\in R\\Rightarrow a^2+b^2\\ge2ab\\quad(\\text{当且仅当 }a=b\\text{ 时取等号})$$'
      },
      {
        label: '四种平均数',
        latex:
          '$$\\begin{gathered}H_n=\\frac{n}{\\sum_{i=1}^{n}\\frac{1}{x_i}}=\\frac{n}{\\frac{1}{x_1}+\\frac{1}{x_2}+\\cdots+\\frac{1}{x_n}}\\\\G_n=\\sqrt[n]{\\prod_{i=1}^{n}x_i}=\\sqrt[n]{x_1x_2\\cdots x_n}\\\\A_n=\\frac{1}{n}\\sum_{i=1}^{n}x_i=\\frac{x_1+x_2+\\cdots+x_n}{n}\\\\Q_n=\\sqrt{\\frac{\\sum_{i=1}^{n}x_i^2}{n}}=\\sqrt{\\frac{x_1^2+x_2^2+\\cdots+x_n^2}{n}}\\\\H_n\\le G_n\\le A_n\\le Q_n\\end{gathered}$$'
      }
    ]
  },
  {
    key: 'integral',
    name: '积分',
    items: [
      { label: '幂函数导数', latex: '$\\frac{d}{dx}x^n=nx^{n-1}$' },
      { label: '指数函数导数', latex: '$\\frac{d}{dx}e^{ax}=ae^{ax}$' },
      { label: '对数函数导数', latex: '$\\frac{d}{dx}\\ln(x)=\\frac{1}{x}$' },
      { label: '正弦导数', latex: '$\\frac{d}{dx}\\sin x=\\cos x$' },
      { label: '余弦导数', latex: '$\\frac{d}{dx}\\cos x=-\\sin x$' },
      { label: '常数积分', latex: '$\\int k\\,dx=kx+C$' },
      { label: '正切导数', latex: '$\\frac{d}{dx}\\tan x=\\sec^2x$' },
      { label: '余切导数', latex: '$\\frac{d}{dx}\\cot x=-\\csc^2x$' },
      { label: '倒数积分', latex: '$\\int\\frac{1}{x}\\,dx=\\ln|x|+C$' },
      { label: '反正弦积分', latex: '$\\int\\frac{1}{\\sqrt{1-x^2}}\\,dx=\\arcsin x+C$' },
      { label: '反正切积分', latex: '$\\int\\frac{1}{1+x^2}\\,dx=\\arctan x+C$' },
      { label: '分部积分', latex: '$\\int u\\frac{dv}{dx}\\,dx=uv-\\int\\frac{du}{dx}v\\,dx$' },
      { label: '傅里叶反变换', latex: '$f(x)=\\int_{-\\infty}^{+\\infty}\\hat f(\\xi)e^{2\\pi i\\xi x}\\,d\\xi$' },
      { label: '幂函数积分', latex: '$\\int x^\\mu\\,dx=\\frac{x^{\\mu+1}}{\\mu+1}+C,\\ (\\mu\\ne-1)$' }
    ]
  },
  {
    key: 'matrix',
    name: '矩阵',
    items: [
      { label: '二阶单位矩阵', latex: '$\\begin{pmatrix}1&0\\\\0&1\\end{pmatrix}$' },
      { label: '三阶矩阵', latex: '$\\begin{pmatrix}a_{11}&a_{12}&a_{13}\\\\a_{21}&a_{22}&a_{23}\\\\a_{31}&a_{32}&a_{33}\\end{pmatrix}$' },
      { label: '一般矩阵', latex: '$\\begin{pmatrix}a_{11}&\\cdots&a_{1n}\\\\\\vdots&\\ddots&\\vdots\\\\a_{m1}&\\cdots&a_{mn}\\end{pmatrix}$' },
      { label: '对称反对称', latex: '$$\\begin{gathered}A=A^T\\\\A=-A^T\\end{gathered}$$' },
      { label: '零矩阵', latex: '$O=\\begin{bmatrix}0&0&\\cdots&0\\\\0&0&\\cdots&0\\\\\\vdots&\\vdots&\\ddots&\\vdots\\\\0&0&\\cdots&0\\end{bmatrix}$' },
      {
        label: '矩阵记号',
        latex: '$A_{m\\times n}=\\begin{bmatrix}a_{11}&a_{12}&\\cdots&a_{1n}\\\\a_{21}&a_{22}&\\cdots&a_{2n}\\\\\\vdots&\\vdots&\\ddots&\\vdots\\\\a_{m1}&a_{m2}&\\cdots&a_{mn}\\end{bmatrix}=[a_{ij}]$'
      },
      {
        label: '矩阵乘法',
        latex:
          '$$\\begin{gathered}A=[a_{ij}]_{m\\times n},\\ B=[b_{ij}]_{n\\times s}\\\\c_{ij}=\\sum_{k=1}^{n}a_{ik}b_{kj}\\\\C=AB=[c_{ij}]_{m\\times s}=\\left[\\sum_{k=1}^{n}a_{ik}b_{kj}\\right]_{m\\times s}\\end{gathered}$$'
      },
      {
        label: '向量叉乘',
        latex: '$\\mathbf V_1\\times\\mathbf V_2=\\begin{vmatrix}\\mathbf i&\\mathbf j&\\mathbf k\\\\\\frac{\\partial X}{\\partial u}&\\frac{\\partial Y}{\\partial u}&0\\\\\\frac{\\partial X}{\\partial v}&\\frac{\\partial Y}{\\partial v}&0\\end{vmatrix}$'
      }
    ]
  },
  {
    key: 'trigonometry',
    name: '三角',
    items: [
      { label: '欧拉形式', latex: '$e^{i\\theta}$' },
      { label: '余角', latex: '$\\left(\\frac{\\pi}{2}-\\theta\\right)$' },
      { label: '正弦半角', latex: '$\\sin^2\\frac{\\alpha}{2}=\\frac{1-\\cos\\alpha}{2}$' },
      { label: '余弦半角', latex: '$\\cos^2\\frac{\\alpha}{2}=\\frac{1+\\cos\\alpha}{2}$' },
      { label: '正切半角', latex: '$\\tan\\frac{\\alpha}{2}=\\frac{\\sin\\alpha}{1+\\cos\\alpha}$' },
      { label: '正弦和差化积', latex: '$\\sin\\alpha+\\sin\\beta=2\\sin\\frac{\\alpha+\\beta}{2}\\cos\\frac{\\alpha-\\beta}{2}$' },
      { label: '正弦差化积', latex: '$\\sin\\alpha-\\sin\\beta=2\\cos\\frac{\\alpha+\\beta}{2}\\sin\\frac{\\alpha-\\beta}{2}$' },
      { label: '余弦和化积', latex: '$\\cos\\alpha+\\cos\\beta=2\\cos\\frac{\\alpha+\\beta}{2}\\cos\\frac{\\alpha-\\beta}{2}$' },
      { label: '余弦差化积', latex: '$\\cos\\alpha-\\cos\\beta=-2\\sin\\frac{\\alpha+\\beta}{2}\\sin\\frac{\\alpha-\\beta}{2}$' },
      { label: '余弦定理', latex: '$a^2=b^2+c^2-2bc\\cos A$' },
      { label: '正弦定理', latex: '$\\frac{\\sin A}{a}=\\frac{\\sin B}{b}=\\frac{\\sin C}{c}=\\frac{1}{2R}$' },
      { label: '诱导公式一', latex: '$\\sin\\left(\\frac{\\pi}{2}-\\alpha\\right)=\\cos\\alpha$' },
      { label: '诱导公式二', latex: '$\\sin\\left(\\frac{\\pi}{2}+\\alpha\\right)=\\cos\\alpha$' }
    ]
  },
  {
    key: 'statistics',
    name: '统计',
    items: [
      { label: '组合数', latex: '$C_r^n$' },
      { label: '组合数阶乘式', latex: '$\\frac{n!}{r!(n-r)!}$' },
      { label: '求和', latex: '$\\sum_{i=1}^{n}X_i$' },
      { label: '平方和', latex: '$\\sum_{i=1}^{n}X_i^2$' },
      { label: '样本序列', latex: '$X_1,\\cdots,X_n$' },
      { label: '标准化', latex: '$\\frac{x-\\mu}{\\sigma}$' },
      { label: '离差平方和', latex: '$\\sum_{i=1}^{n}(X_i-\\overline X)^2$' },
      { label: '独立事件', latex: '$\\text{若 }P(AB)=P(A)P(B),\\ \\text{则 }P(A\\mid B)=P(A)$' },
      { label: '二项分布', latex: '$P(E)=\\binom{n}{k}p^k(1-p)^{n-k}$' },
      { label: '极限概率', latex: '$P(A)=\\lim_{n\\to\\infty}f_n(A)$' },
      { label: '无穷并事件', latex: '$P\\left(\\bigcup_{i=1}^{+\\infty}A_i\\right)=\\prod_{i=1}^{+\\infty}P(A_i)$' },
      { label: '概率边界', latex: '$$\\begin{gathered}P(\\varnothing)=0\\\\P(S)=1\\end{gathered}$$' },
      { label: '概率非负性', latex: '$\\forall A\\in S,\\ P(A)\\ge0$' },
      { label: '有限并事件', latex: '$P\\left(\\bigcup_{i=1}^{n}A_i\\right)=\\prod_{i=1}^{n}P(A_i)$' },
      {
        label: '超几何分布',
        latex: '$$\\begin{gathered}S=\\binom{N}{n},\\ A_k=\\binom{M}{k}\\binom{N-M}{n-k}\\\\P(A_k)=\\frac{\\binom{M}{k}\\binom{N-M}{n-k}}{\\binom{N}{n}}\\end{gathered}$$'
      },
      { label: '排列数', latex: '$$\\begin{gathered}P_n=n!\\\\A_n^k=\\frac{n!}{(n-k)!}\\end{gathered}$$' }
    ]
  },
  {
    key: 'sequence',
    name: '数列',
    items: [
      { label: '等比通项', latex: '$a_n=a_1q^{n-1}$' },
      { label: '等差通项', latex: '$a_n=a_1+(n-1)d$' },
      { label: '等差求和一', latex: '$S_n=na_1+\\frac{n(n-1)}{2}d$' },
      { label: '等差求和二', latex: '$S_n=\\frac{n(a_1+a_n)}{2}$' },
      { label: '裂项一', latex: '$\\frac{1}{n(n+k)}=\\frac{1}{k}\\left(\\frac{1}{n}-\\frac{1}{n+k}\\right)$' },
      { label: '裂项二', latex: '$\\frac{1}{n^2-1}=\\frac{1}{2}\\left(\\frac{1}{n-1}-\\frac{1}{n+1}\\right)$' },
      { label: '裂项三', latex: '$\\frac{1}{4n^2-1}=\\frac{1}{2}\\left(\\frac{1}{2n-1}-\\frac{1}{2n+1}\\right)$' },
      {
        label: '复合裂项',
        latex: '$$\\begin{gathered}\\frac{1}{4n^2-1}=\\frac{1}{2}\\left(\\frac{1}{2n-1}-\\frac{1}{2n+1}\\right)\\\\\\frac{n+1}{n(n-1)2^n}=\\frac{1}{(n-1)2^{n-1}}-\\frac{1}{n2^n}\\end{gathered}$$'
      },
      { label: '等差数列相加', latex: '$\\text{若 }\\{a_n\\},\\{b_n\\}\\text{ 为等差数列，则 }\\{a_n+b_n\\}\\text{ 为等差数列}$' },
      { label: '二项式展开', latex: '$(1+x)^n=1+\\frac{nx}{1!}+\\frac{n(n-1)x^2}{2!}+\\cdots$' }
    ]
  },
  {
    key: 'physics',
    name: '物理',
    items: [
      { label: '合力平衡', latex: '$\\sum \\vec F_i=\\frac{d\\vec v}{dt}=0$' },
      { label: '牛顿第二定律', latex: '$\\vec F=m\\vec a=m\\frac{d^2\\vec r}{dt^2}$' },
      { label: '作用反作用', latex: '$\\vec F_{12}=-\\vec F_{21}$' },
      { label: '引力势能', latex: '$E_p=-\\frac{GMm}{r}$' },
      { label: '库仑定律', latex: '$\\vec F=k\\frac{Qq}{r^2}\\hat r$' },
      { label: '静电场环路', latex: '$\\oint_L\\vec E\\cdot d\\vec l=0$' },
      { label: '毕奥萨伐尔', latex: '$d\\vec B=\\frac{\\mu_0}{4\\pi}\\frac{Id\\vec l\\times\\vec r}{r^3}=\\frac{\\mu_0}{4\\pi}\\frac{Idl\\sin\\theta}{r^2}$' },
      { label: '安培力', latex: '$d\\vec F=Id\\vec l\\times\\vec B$' },
      { label: '感应电动势', latex: '$E=n\\frac{\\Delta\\Phi}{\\Delta t}$' },
      { label: '电通量', latex: '$\\Phi_e=\\oint\\vec E\\cdot d\\vec S=\\frac{1}{\\varepsilon_0}\\sum q$' },
      { label: '法拉第定律', latex: '$\\oint\\vec E\\cdot d\\vec l=-\\frac{d\\phi_B}{dt}$' },
      { label: '安培麦克斯韦定律', latex: '$\\oint\\vec B\\cdot d\\vec l=\\mu_0I+\\mu_0I_d$' },
      { label: '焦耳热', latex: '$Q=I^2Rt$' },
      { label: '万有引力', latex: '$F=G\\frac{Mm}{r^2}$' },
      { label: '光电效应', latex: '$E_k=h\\nu-W_0$' },
      { label: '德布罗意波长', latex: '$\\lambda=\\frac{h}{mv}=\\frac{h}{p}$' },
      { label: '不确定关系', latex: '$\\Delta x\\Delta p\\ge\\frac{h}{4\\pi}$' },
      { label: '长度收缩', latex: '$l=l_0\\sqrt{1-\\left(\\frac{v}{c}\\right)^2}$' },
      { label: '简谐振动', latex: '$y_0=A\\cos(\\omega t+\\varphi_0)$' },
      { label: '波函数', latex: '$y(t)=A\\cos\\left(\\frac{2\\pi x}{\\lambda}+\\varphi\\right)$' },
      {
        label: '麦克斯韦方程组',
        latex:
          '$$\\begin{gathered}\\nabla\\cdot\\mathbf E=\\frac{\\rho}{\\varepsilon_0}\\\\\\nabla\\cdot\\mathbf B=0\\\\\\nabla\\times\\mathbf E=-\\frac{\\partial\\mathbf B}{\\partial t}\\\\\\nabla\\times\\mathbf B=\\mu_0\\mathbf J+\\mu_0\\varepsilon_0\\frac{\\partial\\mathbf E}{\\partial t}\\end{gathered}$$'
      },
      {
        label: '宏观麦克斯韦方程组',
        latex:
          '$$\\begin{gathered}\\nabla\\cdot\\mathbf D=\\rho_f\\\\\\nabla\\cdot\\mathbf B=0\\\\\\nabla\\times\\mathbf E=-\\frac{\\partial\\mathbf B}{\\partial t}\\\\\\nabla\\times\\mathbf H=\\mathbf J_f+\\frac{\\partial\\mathbf D}{\\partial t}\\end{gathered}$$'
      }
    ]
  }
]

export const commonSymbolCategories: FormulaCategory[] = [
  {
    key: 'basicOperation',
    name: '基础运算',
    items: [
      { label: '加号', latex: '$+$' },
      { label: '减号', latex: '$-$' },
      { label: '加减', latex: '$\\pm$' },
      { label: '减加', latex: '$\\mp$' },
      { label: '乘号', latex: '$\\times$' },
      { label: '除号', latex: '$\\div$' },
      { label: '点乘', latex: '$\\cdot$' },
      { label: '星乘', latex: '$\\ast$' },
      { label: '绝对值', latex: '$|x|$' },
      { label: '范数', latex: '$\\|x\\|$' },
      { label: '圆括号', latex: '$(x)$' },
      { label: '方括号', latex: '$[x]$' },
      { label: '花括号', latex: '$\\{x\\}$' },
      { label: '比例', latex: '$a:b$' },
      { label: '省略号', latex: '$\\cdots$' },
      { label: '阶乘', latex: '$n!$' }
    ]
  },
  {
    key: 'fractionRadical',
    name: '分数根式',
    items: [
      { label: '二分之一', latex: '$\\frac{1}{2}$' },
      { label: '三分之一', latex: '$\\frac{1}{3}$' },
      { label: '四分之一', latex: '$\\frac{1}{4}$' },
      { label: '分式', latex: '$\\frac{a}{b}$' },
      { label: '带分式', latex: '$1\\frac{1}{2}$' },
      { label: '负分式', latex: '$-\\frac{a}{b}$' },
      { label: '连分式', latex: '$\\frac{1}{1+\\frac{1}{x}}$' },
      { label: '平方根', latex: '$\\sqrt{x}$' },
      { label: '三次根', latex: '$\\sqrt[3]{x}$' },
      { label: 'n 次根', latex: '$\\sqrt[n]{x}$' },
      { label: '根式加减', latex: '$\\sqrt{a}\\pm\\sqrt{b}$' },
      { label: '根式分母', latex: '$\\frac{1}{\\sqrt{x}}$' },
      { label: '根式模板', latex: '$\\sqrt{a^2+b^2}$' },
      { label: '根号外系数', latex: '$a\\sqrt{b}$' }
    ]
  },
  {
    key: 'scriptPower',
    name: '指数上下标',
    items: [
      { label: '平方', latex: '$x^2$' },
      { label: '立方', latex: '$x^3$' },
      { label: 'n 次方', latex: '$x^n$' },
      { label: '负指数', latex: '$x^{-1}$' },
      { label: '分数指数', latex: '$x^{\\frac{1}{2}}$' },
      { label: '指数函数', latex: '$e^x$' },
      { label: '幂函数', latex: '$a^x$' },
      { label: '下标', latex: '$x_i$' },
      { label: '双下标', latex: '$a_{ij}$' },
      { label: '上下标', latex: '$x_i^2$' },
      { label: '序列下标', latex: '$a_{n+1}$' },
      { label: '科学计数法', latex: '$a\\times10^n$' },
      { label: '极限趋近', latex: '$x\\to0$' },
      { label: '向量分量', latex: '$\\vec a_i$' }
    ]
  },
  {
    key: 'relationCompare',
    name: '关系比较',
    items: [
      { label: '等于', latex: '$=$' },
      { label: '不等于', latex: '$\\ne$' },
      { label: '大于', latex: '$>$' },
      { label: '小于', latex: '$<$' },
      { label: '大于等于', latex: '$\\ge$' },
      { label: '小于等于', latex: '$\\le$' },
      { label: '约等于', latex: '$\\approx$' },
      { label: '恒等于', latex: '$\\equiv$' },
      { label: '正比于', latex: '$\\propto$' },
      { label: '相似', latex: '$\\sim$' },
      { label: '不相似', latex: '$\\nsim$' },
      { label: '全等', latex: '$\\cong$' },
      { label: '趋近于', latex: '$\\to$' },
      { label: '远小于', latex: '$\\ll$' },
      { label: '远大于', latex: '$\\gg$' },
      { label: '对应', latex: '$\\leftrightarrow$' }
    ]
  },
  {
    key: 'setLogic',
    name: '集合逻辑',
    items: [
      { label: '属于', latex: '$\\in$' },
      { label: '不属于', latex: '$\\notin$' },
      { label: '包含于', latex: '$\\subset$' },
      { label: '真包含于', latex: '$\\subsetneqq$' },
      { label: '包含', latex: '$\\supset$' },
      { label: '真包含', latex: '$\\supsetneqq$' },
      { label: '并集', latex: '$\\cup$' },
      { label: '交集', latex: '$\\cap$' },
      { label: '空集', latex: '$\\varnothing$' },
      { label: '全集', latex: '$U$' },
      { label: '补集', latex: '$\\complement A$' },
      { label: '且', latex: '$\\land$' },
      { label: '或', latex: '$\\lor$' },
      { label: '非', latex: '$\\lnot$' },
      { label: '任意', latex: '$\\forall$' },
      { label: '存在', latex: '$\\exists$' }
    ]
  },
  {
    key: 'basicGeometry',
    name: '几何基础',
    items: [
      { label: '角', latex: '$\\angle A$' },
      { label: '角度', latex: '$60^\\circ$' },
      { label: '三角形', latex: '$\\triangle ABC$' },
      { label: '平行', latex: '$AB\\parallel CD$' },
      { label: '垂直', latex: '$AB\\perp CD$' },
      { label: '圆', latex: '$\\odot O$' },
      { label: '弧', latex: '$\\overset{\\frown}{AB}$' },
      { label: '线段', latex: '$\\overline{AB}$' },
      { label: '射线', latex: '$\\overrightarrow{AB}$' },
      { label: '直线', latex: '$\\overleftrightarrow{AB}$' },
      { label: '向量', latex: '$\\vec{AB}$' },
      { label: '相似', latex: '$\\triangle ABC\\sim\\triangle DEF$' },
      { label: '全等', latex: '$\\triangle ABC\\cong\\triangle DEF$' },
      { label: '面积', latex: '$S=\\frac{1}{2}ah$' },
      { label: '周长', latex: '$C=2\\pi r$' },
      { label: '体积', latex: '$V=abh$' }
    ]
  },
  {
    key: 'statisticsSum',
    name: '统计求和',
    items: [
      { label: '求和', latex: '$\\sum_{i=1}^{n}x_i$' },
      { label: '平方和', latex: '$\\sum_{i=1}^{n}x_i^2$' },
      { label: '连乘', latex: '$\\prod_{i=1}^{n}x_i$' },
      { label: '平均数', latex: '$\\frac{x_1+x_2+\\cdots+x_n}{n}$' },
      { label: '样本均值', latex: '$\\overline{x}$' },
      { label: '总体均值', latex: '$\\mu$' },
      { label: '方差', latex: '$s^2=\\frac{1}{n}\\sum_{i=1}^{n}(x_i-\\overline{x})^2$' },
      { label: '标准差', latex: '$s=\\sqrt{\\frac{1}{n}\\sum_{i=1}^{n}(x_i-\\overline{x})^2}$' },
      { label: '百分比', latex: '$p\\%$' },
      { label: '组合数', latex: '$\\binom{n}{k}$' },
      { label: '排列数', latex: '$A_n^k$' },
      { label: '概率', latex: '$P(A)$' },
      { label: '条件概率', latex: '$P(A\\mid B)$' },
      { label: '正态分布', latex: '$N(\\mu,\\sigma^2)$' }
    ]
  },
  {
    key: 'special',
    name: '常用特殊',
    items: [
      { label: '无穷', latex: '$\\infty$' },
      { label: '正无穷', latex: '$+\\infty$' },
      { label: '负无穷', latex: '$-\\infty$' },
      { label: '因此', latex: '$\\therefore$' },
      { label: '因为', latex: '$\\because$' },
      { label: '推出', latex: '$\\Rightarrow$' },
      { label: '双向推出', latex: '$\\Leftrightarrow$' },
      { label: '度', latex: '$^\\circ$' },
      { label: '摄氏度', latex: '$^\\circ\\mathrm{C}$' },
      { label: '上箭头', latex: '$\\uparrow$' },
      { label: '下箭头', latex: '$\\downarrow$' },
      { label: '左箭头', latex: '$\\leftarrow$' },
      { label: '右箭头', latex: '$\\rightarrow$' },
      { label: '左右箭头', latex: '$\\leftrightarrow$' },
      { label: '空白占位', latex: '$\\square$' },
      { label: '星号', latex: '$\\star$' }
    ]
  }
]

export const phoneticSymbols: FormulaItem[] = [
  { label: 'i:', latex: '/i:/' },
  { label: 'ɪ', latex: '/ɪ/' },
  { label: 'e', latex: '/e/' },
  { label: 'æ', latex: '/æ/' },
  { label: 'ɑ:', latex: '/ɑ:/' },
  { label: 'ɔ:', latex: '/ɔ:/' },
  { label: 'u:', latex: '/u:/' },
  { label: 'ə', latex: '/ə/' },
  { label: 'θ', latex: '/θ/' },
  { label: 'ð', latex: '/ð/' },
  { label: 'ʃ', latex: '/ʃ/' },
  { label: 'tʃ', latex: '/tʃ/' }
]
