var contentString = document.getElementById("content").textContent;
document.getElementById("content").innerHTML = contentString;

var menuString = document.getElementById("lnav").textContent;
document.getElementById("lnav").innerHTML = menuString;

var contentHeading = document.getElementById("content");
var all_heading = contentHeading.querySelectorAll("h1, h2, h3, h4, h5");
var rnav = document.getElementById("rnav");
var rnav_ul = rnav.querySelector("ul");

all_heading.forEach(function (h) {
    var textnode = document.createTextNode(h.innerHTML);
    var li = document.createElement("LI");
    var a = document.createElement("A");
    a.setAttribute("href", h.innerHTML.replace(/ /g, "_"));
    console.log(h.previousSibling.href);
    li.appendChild(textnode);
    li.addEventListener("click", function () {
        h.scrollIntoView({
            behavior: "smooth",
            block: "end",
            inline: "nearest",
        });
    });
    rnav_ul.appendChild(li);
    h.addEventListener("click", function () {
        if (h.previousElementSibling.children.length > 0) {
            let hash = h.previousElementSibling.children[0].getAttribute(
                "href"
            );
            if (hash) {
                history.pushState(null, null, hash);
            }
        }
    });
});

document.getElementById("content").addEventListener("scroll", function () {
    console.log("Scrolling");
});
