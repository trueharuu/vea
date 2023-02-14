let f=(v)=>fetch(`https://api.urbandictionary.com/v0/define?term=${encodeURIComponent(v)}`).then(x=>x.json()).then(({list})=>list.map(x=>`**${x.word}** - *@${x.author}*
${x.definition}`).slice(0, 5).join(`

`));