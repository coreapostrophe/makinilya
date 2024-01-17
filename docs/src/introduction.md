# Introduction

<style>
 code {
 white-space : pre-wrap !important;
 }
</style>

Decision making has got to be one of the most time-consuming aspects of fiction writing. Dialogues, where the story goes, how each plot line are introduced, these are few of the numerous questions we asked ourselves whenever we traverse the treacherous journey of crafting a narrative.

However, from all of the questions that needs an authorial verdict, nothing else brings more anguish than those that revolves around **naming**. Places, characters, objects; anything that needs a name requires an extensive knowledge of their history and characteristics, for the name to be meaningful. It is for this reason why deciding a name is often a difficult endeavor.

Nothing is set in place when starting a narrative. Every single plot devices is thoroughly developed and built as the draft progresses and it's almost impossible to avoid rehashes of old scenes and characters. So why should we write things as if they're final?

Makinilya aims to introduce probationary references to your draft, those that could be easily replaced at any time. It makes use of clever layouting techniques such as "string interpolation" to support placeholders in the manuscript.

So instead of writing,

```plaintext
Hi, my name is Core. I'm a 22 years old professional working in the software industry.
```

We could write,

```plaintext
Hi, my name is {{ author.name }}. I'm a {{ author.age }} years old professional working in the software industry. 
```

This is a less strenuous way of writing narratives, albeit an admittedly verbose one.
