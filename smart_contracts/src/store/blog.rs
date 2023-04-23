struct BlogPost<'a>{
    title: &'a String,
    content: &'a String,
    date: &'a String,
    author: &'a String,
}

const blog_1: BlogPost = BlogPost {
    title: "First blog post",
    content: "Content stuff",
    date: "today",
    authoer: "wrx",
};

const blogs: [BlogPost] = [blog_1,];

fn print_blogs (blogList: [BlogPost]) {
    let i = 0;
    loop {
        println!("{}", blogList[i].title);
        i += 1;
        if i > i.len() {
            break
        };
    };

}

const printBlogs: fn([BlogPost]) = print_blogs;


