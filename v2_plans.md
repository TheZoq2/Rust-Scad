#Desires

I want to be able to have optional "parameters" on objects. For example,
you should be able to add the 'center' parameter to cubes.

It would also be nice to allow objects of the same type but with different data.
For example, colors can either use color name or rgb value. The current system 
does not support this.

As far as possible, it would be nice if the new version is backwards compatible
with the old one. 

Additionally, one thing I would like to do is to allow you to also use the
parameters to implement behaviour not present in scad itself. For example,
a cube can only be centered on all axis at the same time in scad, but
a nice feature to have would be to allow centering on only one or two axis.

#Implementation
The best idea is probably to replace the scad_element enum with a struct. 

Backwards compatibility can be achieved using the macro, however, it might be 
tricky to do that while allowing 
