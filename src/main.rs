use competition::pattern::Pattern;
//思路就是: 先生成最基本的模型, 然后进行处理, 得到结果
//最基本的模型见其他模块的注释
//怎么处理: 先给定一个比率0到1, 每个学校的评委数量乘以这个数,约到整数,也就是从每个学校固定比例抽取一部分评委,遍历评委和论文,一旦论文和评委来自同一所学校, 在把这篇论文随机地分给其他学校评委, 以此类推, 最后全部打印出来
//什么编程语言都能做, 只不过python会非常简单, 不过我还是喜欢Rust
fn main() {
    Pattern::default();
}