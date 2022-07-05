module param_array # (
    parameter WIDTH = 32'd32,
    parameter DEPTH = 32'd1024
);
    reg [WIDTH-1:0] ram [DEPTH-1:0];
endmodule
