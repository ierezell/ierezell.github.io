let J=0,I=null,K=`undefined`,Q=`boolean`,R=`string`,Y=55,O=1,T=`Object`,L=`utf-8`,P=`number`,W=4,N=`function`,G=Array,S=Array.isArray,M=Error,U=JSON,X=Object,V=Reflect,H=undefined;var A=(async(a,b)=>{if(typeof Response===N&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===N){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var u=(a=>{const b=typeof a;if(b==P||b==Q||a==I){return `${a}`};if(b==R){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==I){return `Symbol`}else{return `Symbol(${b})`}};if(b==N){const b=a.name;if(typeof b==R&&b.length>J){return `Function(${b})`}else{return `Function`}};if(S(a)){const b=a.length;let c=`[`;if(b>J){c+=u(a[J])};for(let d=O;d<b;d++){c+=`, `+ u(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>O){d=c[O]}else{return toString.call(a)};if(d==T){try{return `Object(`+ U.stringify(a)+ `)`}catch(a){return T}};if(a instanceof M){return `${a.name}: ${a.message}\n${a.stack}`};return d});var C=((a,b)=>{});var p=(a=>{if(d===b.length)b.push(b.length+ O);const c=d;d=b[c];b[c]=a;return c});var f=(a=>{const b=c(a);e(a);return b});var x=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__h167e74c6b52f47b5(b,c,p(d))});function z(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(p(b))}}var y=((a,b)=>{if(a===J){return c(b)}else{return r(a,b)}});var o=(()=>{if(n===I||n.byteLength===J){n=new Int32Array(a.memory.buffer)};return n});var m=(a=>a===H||a===I);var c=(a=>b[a]);var F=(async(b)=>{if(a!==H)return a;if(typeof b===K){b=new URL(`web-096ab58452d04e2b26dabc21d2fcb15e146e98737e15ae2ca702536c10e11625b815aa96d2cbfd6be2c18659ff76a318_bg.wasm`,import.meta.url)};const c=B();if(typeof b===R||typeof Request===N&&b instanceof Request||typeof URL===N&&b instanceof URL){b=fetch(b)};C(c);const {instance:d,module:e}=await A(await b,c);return D(d,e)});var E=(b=>{if(a!==H)return a;const c=B();C(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return D(d,b)});var D=((b,c)=>{a=b.exports;F.__wbindgen_wasm_module=c;s=I;n=I;h=I;a.__wbindgen_start();return a});var t=(()=>{if(s===I||s.byteLength===J){s=new Float64Array(a.memory.buffer)};return s});var B=(()=>{const b={};b.wbg={};b.wbg.__wbg_body_64abc9aba1891e91=(a=>{const b=c(a).body;return m(b)?J:p(b)});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===R;return b});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==O){b.a=J;return !0};const c=!1;return c});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===N;return b});b.wbg.__wbg_call_90c26b09837aba1c=function(){return z(((a,b)=>{const d=c(a).call(c(b));return p(d)}),arguments)};b.wbg.__wbg_get_7b48513de5dc5ea4=function(){return z(((a,b)=>{const d=V.get(c(a),c(b));return p(d)}),arguments)};b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===R?e:H;var h=m(f)?J:l(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var i=g;o()[b/W+ O]=i;o()[b/W+ J]=h});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return p(b)});b.wbg.__wbg_self_f0e34d89f33b99fd=function(){return z((()=>{const a=self.self;return p(a)}),arguments)};b.wbg.__wbg_window_d3b084224f4774d7=function(){return z((()=>{const a=window.window;return p(a)}),arguments)};b.wbg.__wbg_globalThis_9caa27ff917c6860=function(){return z((()=>{const a=globalThis.globalThis;return p(a)}),arguments)};b.wbg.__wbg_global_35dfdd59a4da3e74=function(){return z((()=>{const a=global.global;return p(a)}),arguments)};b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===H;return b});b.wbg.__wbg_newnoargs_c62ea9419c21fbac=((a,b)=>{var c=y(a,b);const d=new Function(c);return p(d)});b.wbg.__wbg_decodeURI_1e508fc8ed99cae7=function(){return z(((a,b)=>{var c=y(a,b);const d=decodeURI(c);return p(d)}),arguments)};b.wbg.__wbg_call_5da1969d7cd31ccd=function(){return z(((a,b,d)=>{const e=c(a).call(c(b),c(d));return p(e)}),arguments)};b.wbg.__wbg_is_ff7acd231c75c0e4=((a,b)=>{const d=X.is(c(a),c(b));return d});b.wbg.__wbg_set_759f75cd92b612d2=function(){return z(((a,b,d)=>{const e=V.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbg_exec_42513e2d2ddabd95=((a,b,d)=>{var e=y(b,d);const f=c(a).exec(e);return m(f)?J:p(f)});b.wbg.__wbg_new_e145ee1b0ed9b4aa=((a,b,c,d)=>{var e=y(a,b);var f=y(c,d);const g=new RegExp(e,f);return p(g)});b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=r(a,b);return p(c)});b.wbg.__wbg_error_e60eff06f24ab7a4=(a=>{console.error(c(a))});b.wbg.__wbg_remove_0d26d36fd4f25c4e=(a=>{c(a).remove()});b.wbg.__wbg_setdata_86ad1e8da020aa68=((a,b,d)=>{var e=y(b,d);c(a).data=e});b.wbg.__wbg_childNodes_a5762b4b3e073cf6=(a=>{const b=c(a).childNodes;return p(b)});b.wbg.__wbg_length_f845c1c304d9837a=(a=>{const b=c(a).length;return b});b.wbg.__wbg_remove_c6ba26a0a6906129=function(){return z(((a,b,d)=>{var e=y(b,d);c(a).remove(e)}),arguments)};b.wbg.__wbg_add_e0f3c5b6e421c311=function(){return z(((a,b,d)=>{var e=y(b,d);c(a).add(e)}),arguments)};b.wbg.__wbg_document_d609202d16c38224=(a=>{const b=c(a).document;return m(b)?J:p(b)});b.wbg.__wbg_new_8b18a325932736b8=function(){return z((()=>{const a=new Range();return p(a)}),arguments)};b.wbg.__wbg_createComment_529b047c02bbe600=((a,b,d)=>{var e=y(b,d);const f=c(a).createComment(e);return p(f)});b.wbg.__wbg_target_52ddf6955f636bf5=(a=>{const b=c(a).target;return m(b)?J:p(b)});b.wbg.__wbg_composedPath_12a068e57a98cf90=(a=>{const b=c(a).composedPath();return p(b)});b.wbg.__wbg_get_f01601b5a68d10e3=((a,b)=>{const d=c(a)[b>>>J];return p(d)});b.wbg.__wbindgen_is_falsy=(a=>{const b=!c(a);return b});b.wbg.__wbg_cancelBubble_976cfdf7ac449a6c=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_instanceof_ShadowRoot_0bd39e89ab117f86=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_09eee5e3d9cf59a1=(a=>{const b=c(a).host;return p(b)});b.wbg.__wbindgen_is_null=(a=>{const b=c(a)===I;return b});b.wbg.__wbg_createDocumentFragment_1c6d6aeeb8a8eb2e=(a=>{const b=c(a).createDocumentFragment();return p(b)});b.wbg.__wbg_location_176c34e89c2c9d80=(a=>{const b=c(a).location;return p(b)});b.wbg.__wbg_requestAnimationFrame_74309aadebde12fa=function(){return z(((a,b)=>{const d=c(a).requestAnimationFrame(c(b));return d}),arguments)};b.wbg.__wbg_removeEventListener_66ee1536a0b32c11=function(){return z(((a,b,d,e)=>{var f=y(b,d);c(a).removeEventListener(f,c(e))}),arguments)};b.wbg.__wbg_log_a4530b4fe289336f=(a=>{console.log(c(a))});b.wbg.__wbg_warn_f260f49434e45e62=(a=>{console.warn(c(a))});b.wbg.__wbg_classList_82893a9100db6428=(a=>{const b=c(a).classList;return p(b)});b.wbg.__wbg_createTextNode_7ff0c034b2855f66=((a,b,d)=>{var e=y(b,d);const f=c(a).createTextNode(e);return p(f)});b.wbg.__wbg_newwithbase_f4989aa5bbd5cc29=function(){return z(((a,b,c,d)=>{var e=y(a,b);var f=y(c,d);const g=new URL(e,f);return p(g)}),arguments)};b.wbg.__wbg_origin_aab6d2be79bcec84=((b,d)=>{const e=c(d).origin;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;o()[b/W+ O]=h;o()[b/W+ J]=f});b.wbg.__wbg_pathname_aeafa820be91c325=((b,d)=>{const e=c(d).pathname;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;o()[b/W+ O]=h;o()[b/W+ J]=f});b.wbg.__wbg_search_f6e95882a48d3f69=((b,d)=>{const e=c(d).search;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;o()[b/W+ O]=h;o()[b/W+ J]=f});b.wbg.__wbg_searchParams_00f98167a3c8c4da=(a=>{const b=c(a).searchParams;return p(b)});b.wbg.__wbg_iterator_db7ca081358d4fb2=(()=>{const a=Symbol.iterator;return p(a)});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==I;return d});b.wbg.__wbg_next_9b877f231f476d01=(a=>{const b=c(a).next;return p(b)});b.wbg.__wbg_next_6529ee0cca8d57ed=function(){return z((a=>{const b=c(a).next();return p(b)}),arguments)};b.wbg.__wbg_done_5fe336b092d60cf2=(a=>{const b=c(a).done;return b});b.wbg.__wbg_value_0c248a78fdc8e19f=(a=>{const b=c(a).value;return p(b)});b.wbg.__wbg_isArray_74fb723e24f76012=(a=>{const b=S(c(a));return b});b.wbg.__wbg_hash_0087751acddc8f2a=((b,d)=>{const e=c(d).hash;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;o()[b/W+ O]=h;o()[b/W+ J]=f});b.wbg.__wbg_addEventListener_374cbfd2bbc19ccf=function(){return z(((a,b,d,e,f)=>{var g=y(b,d);c(a).addEventListener(g,c(e),c(f))}),arguments)};b.wbg.__wbg_getElementById_65b9547a428b5eb4=((a,b,d)=>{var e=y(b,d);const f=c(a).getElementById(e);return m(f)?J:p(f)});b.wbg.__wbg_scrollIntoView_3de22d537ed95550=(a=>{c(a).scrollIntoView()});b.wbg.__wbg_scrollTo_eb21c4452d7b3cd6=((a,b,d)=>{c(a).scrollTo(b,d)});b.wbg.__wbindgen_jsval_eq=((a,b)=>{const d=c(a)===c(b);return d});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===P?d:H;t()[a/8+ O]=m(e)?J:e;o()[a/W+ J]=!m(e)});b.wbg.__wbg_defaultPrevented_ae7d433108dd159d=(a=>{const b=c(a).defaultPrevented;return b});b.wbg.__wbg_button_cd87b6dabbde9631=(a=>{const b=c(a).button;return b});b.wbg.__wbg_metaKey_2a8dbd51a3f59e9c=(a=>{const b=c(a).metaKey;return b});b.wbg.__wbg_altKey_c6c2a7e797d9a669=(a=>{const b=c(a).altKey;return b});b.wbg.__wbg_ctrlKey_643b17aaac67db50=(a=>{const b=c(a).ctrlKey;return b});b.wbg.__wbg_shiftKey_8fb7301f56e7e01c=(a=>{const b=c(a).shiftKey;return b});b.wbg.__wbg_length_1009b1af0c481d7b=(a=>{const b=c(a).length;return b});b.wbg.__wbg_href_829df0adc5a7228a=((b,d)=>{const e=c(d).href;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;o()[b/W+ O]=h;o()[b/W+ J]=f});b.wbg.__wbg_target_b68f65aba6338cfb=((b,d)=>{const e=c(d).target;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;o()[b/W+ O]=h;o()[b/W+ J]=f});b.wbg.__wbg_getAttribute_bff489553dd803cc=((b,d,e,f)=>{var h=y(e,f);const i=c(d).getAttribute(h);var j=m(i)?J:l(i,a.__wbindgen_malloc,a.__wbindgen_realloc);var k=g;o()[b/W+ O]=k;o()[b/W+ J]=j});b.wbg.__wbg_preventDefault_7f821f72e7c6b5d4=(a=>{c(a).preventDefault()});b.wbg.__wbindgen_boolean_get=(a=>{const b=c(a);const d=typeof b===Q?(b?O:J):2;return d});b.wbg.__wbg_instanceof_HtmlAnchorElement_76fafcefedd51299=(a=>{let b;try{b=c(a) instanceof HTMLAnchorElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_history_80998b7456bf367e=function(){return z((a=>{const b=c(a).history;return p(b)}),arguments)};b.wbg.__wbg_pushState_e159043fce8f87bc=function(){return z(((a,b,d,e,f,g)=>{var h=y(d,e);var i=y(f,g);c(a).pushState(c(b),h,i)}),arguments)};b.wbg.__wbg_replaceState_b51dd62c7235b1ac=function(){return z(((a,b,d,e,f,g)=>{var h=y(d,e);var i=y(f,g);c(a).replaceState(c(b),h,i)}),arguments)};b.wbg.__wbg_pathname_1ab7e82aaa4511ff=function(){return z(((b,d)=>{const e=c(d).pathname;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;o()[b/W+ O]=h;o()[b/W+ J]=f}),arguments)};b.wbg.__wbg_search_9f7ca8896c2d0804=function(){return z(((b,d)=>{const e=c(d).search;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;o()[b/W+ O]=h;o()[b/W+ J]=f}),arguments)};b.wbg.__wbg_parse_3423ec3227d9fe98=function(){return z(((a,b)=>{var c=y(a,b);const d=U.parse(c);return p(d)}),arguments)};b.wbg.__wbg_instanceof_Object_702c4990f4c3db8d=(a=>{let b;try{b=c(a) instanceof X}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_newPlot_f404e70f78b6c842=function(){return z(((a,b,d)=>{var e=y(a,b);const f=Plotly.newPlot(e,c(d));return p(f)}),arguments)};b.wbg.__wbg_length_c8f895dad5ec94df=(a=>{const b=c(a).length;return b});b.wbg.__wbindgen_debug_string=((b,d)=>{const e=u(c(d));const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;o()[b/W+ O]=h;o()[b/W+ J]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new M(r(a,b))});b.wbg.__wbg_then_3ab08cd4fbb91ae9=((a,b)=>{const d=c(a).then(c(b));return p(d)});b.wbg.__wbg_queueMicrotask_4d890031a6a5a50c=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_then_8371cc12cfedc5a2=((a,b,d)=>{const e=c(a).then(c(b),c(d));return p(e)});b.wbg.__wbg_queueMicrotask_adae4bc085237231=(a=>{const b=c(a).queueMicrotask;return p(b)});b.wbg.__wbg_resolve_6e1c6553a82f85b7=(a=>{const b=Promise.resolve(c(a));return p(b)});b.wbg.__wbg_instanceof_Window_3e5cd1f48c152d01=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_text_3686904a1dc2b550=(a=>{const b=c(a).text();return p(b)});b.wbg.__wbg_createElement_fdd5c113cb84539e=function(){return z(((a,b,d)=>{var e=y(b,d);const f=c(a).createElement(e);return p(f)}),arguments)};b.wbg.__wbg_append_962e199b73af5069=function(){return z(((a,b)=>{c(a).append(c(b))}),arguments)};b.wbg.__wbg_setinnerHTML_ce0d6527ce4086f2=((a,b,d)=>{var e=y(b,d);c(a).innerHTML=e});b.wbg.__wbg_hasAttribute_bfb8f7140cf587f1=((a,b,d)=>{var e=y(b,d);const f=c(a).hasAttribute(e);return f});b.wbg.__wbg_removeAttribute_2e200daefb9f3ed4=function(){return z(((a,b,d)=>{var e=y(b,d);c(a).removeAttribute(e)}),arguments)};b.wbg.__wbg_setAttribute_e7b72a5e7cfcb5a3=function(){return z(((a,b,d,e,f)=>{var g=y(b,d);var h=y(e,f);c(a).setAttribute(g,h)}),arguments)};b.wbg.__wbg_before_74a825a7b3d13d06=function(){return z(((a,b)=>{c(a).before(c(b))}),arguments)};b.wbg.__wbg_append_df44ca631c3c1657=function(){return z(((a,b)=>{c(a).append(c(b))}),arguments)};b.wbg.__wbg_addEventListener_9bf60ea8a362e5e4=function(){return z(((a,b,d,e)=>{var f=y(b,d);c(a).addEventListener(f,c(e))}),arguments)};b.wbg.__wbg_name_bbf9c43b9611377a=((b,d)=>{const e=c(d).name;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;o()[b/W+ O]=h;o()[b/W+ J]=f});b.wbg.__wbg_item_312a88e5ba8832a5=((a,b)=>{const d=c(a).item(b>>>J);return m(d)?J:p(d)});b.wbg.__wbg_files_84acf164900adf19=(a=>{const b=c(a).files;return m(b)?J:p(b)});b.wbg.__wbg_origin_595edc88be6e66b8=function(){return z(((b,d)=>{const e=c(d).origin;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;o()[b/W+ O]=h;o()[b/W+ J]=f}),arguments)};b.wbg.__wbg_hash_be2940ca236b5efc=function(){return z(((b,d)=>{const e=c(d).hash;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;o()[b/W+ O]=h;o()[b/W+ J]=f}),arguments)};b.wbg.__wbg_parentNode_92a7017b3a4fad43=(a=>{const b=c(a).parentNode;return m(b)?J:p(b)});b.wbg.__wbg_previousSibling_ef843c512fac0d77=(a=>{const b=c(a).previousSibling;return m(b)?J:p(b)});b.wbg.__wbg_nextSibling_bafccd3347d24543=(a=>{const b=c(a).nextSibling;return m(b)?J:p(b)});b.wbg.__wbg_settextContent_3ebccdd9354e1601=((a,b,d)=>{var e=y(b,d);c(a).textContent=e});b.wbg.__wbg_appendChild_d30e6b83791d04c0=function(){return z(((a,b)=>{const d=c(a).appendChild(c(b));return p(d)}),arguments)};b.wbg.__wbg_cloneNode_405d5ea3f7e0098a=function(){return z((a=>{const b=c(a).cloneNode();return p(b)}),arguments)};b.wbg.__wbg_deleteContents_08069ffe080b9480=function(){return z((a=>{c(a).deleteContents()}),arguments)};b.wbg.__wbg_setEndBefore_2fcd1d853bf5ebfa=function(){return z(((a,b)=>{c(a).setEndBefore(c(b))}),arguments)};b.wbg.__wbg_setStartBefore_5a200b7348513263=function(){return z(((a,b)=>{c(a).setStartBefore(c(b))}),arguments)};b.wbg.__wbindgen_closure_wrapper714=((a,b,c)=>{const d=v(a,b,Y,w);return p(d)});b.wbg.__wbindgen_closure_wrapper718=((a,b,c)=>{const d=v(a,b,58,x);return p(d)});b.wbg.__wbindgen_closure_wrapper1558=((a,b,c)=>{const d=v(a,b,Y,w);return p(d)});b.wbg.__wbindgen_closure_wrapper1879=((a,b,c)=>{const d=v(a,b,90,x);return p(d)});b.wbg.__wbindgen_closure_wrapper4034=((a,b,c)=>{const d=v(a,b,Y,x);return p(d)});return b});var e=(a=>{if(a<132)return;b[a]=d;d=a});var v=((b,c,d,e)=>{const f={a:b,b:c,cnt:O,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=J;try{return e(c,f.b,...b)}finally{if(--f.cnt===J){a.__wbindgen_export_2.get(f.dtor)(c,f.b)}else{f.a=c}}};g.original=f;return g});var l=((a,b,c)=>{if(c===H){const c=j.encode(a);const d=b(c.length,O)>>>J;i().subarray(d,d+ c.length).set(c);g=c.length;return d};let d=a.length;let e=b(d,O)>>>J;const f=i();let h=J;for(;h<d;h++){const b=a.charCodeAt(h);if(b>127)break;f[e+ h]=b};if(h!==d){if(h!==J){a=a.slice(h)};e=c(e,d,d=h+ a.length*3,O)>>>J;const b=i().subarray(e+ h,e+ d);const f=k(a,b);h+=f.written};g=h;return e});var i=(()=>{if(h===I||h.byteLength===J){h=new Uint8Array(a.memory.buffer)};return h});var r=((a,b)=>{a=a>>>J;return q.decode(i().subarray(a,a+ b))});var w=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__h144ac7e5af081a37(b,c)});let a;const b=new G(128).fill(H);b.push(H,I,!0,!1);let d=b.length;let g=J;let h=I;const j=typeof TextEncoder!==K?new TextEncoder(L):{encode:()=>{throw M(`TextEncoder not available`)}};const k=typeof j.encodeInto===N?((a,b)=>j.encodeInto(a,b)):((a,b)=>{const c=j.encode(a);b.set(c);return {read:a.length,written:c.length}});let n=I;const q=typeof TextDecoder!==K?new TextDecoder(L,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw M(`TextDecoder not available`)}};if(typeof TextDecoder!==K){q.decode()};let s=I;export default F;export{E as initSync}