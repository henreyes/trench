;; To run this, load this file into the lisp interpreter, then call
;; (my-top)
;; then type in expressions to be evaluated.

(defun my-assoc (v alist)
    (
        cond
            ((null alist) nil)
            ((eq v (car(car alist) )) (car alist)) 
            (t (my-assoc v (cdr alist)))
    )
)


(defun my-eval (e alist)
    (
        cond 
            ((atom e) (my-eval-atom e alist))
            (t (my-apply (car e) (cdr e) alist))
    )
)


(defun my-eval-atom (e alist)
    (
        cond
            ((numberp e) e)
            ((stringp e) e)
            ((eq e 'NIL) NIL)
            ((eq e 'T) T)
            ((my-assoc e alist) (cdr (my-assoc e alist)))
            (t e)       
    )
)



(defun my-apply (fn args alist)
    (cond ((atom fn) (my-apply-atom fn args alist))
          ( t (my-apply-lambda fn args alist)))
)


(defun my-eval-list (l alist)
    (
        cond
            ((null l) NIL)
            ((null (cdr l)) (my-eval (car l) alist))
            (t (my-eval (car l) alist) (my-eval-list (cdr l) alist))
    )
)



(defun my-apply-lambda (fn args alist)
  (my-eval-list (cdr fn) (my-bind-formals (car fn) args alist))
)



(defun my-bind-formals (formals actuals alist)
    (
        cond
            ((null formals) alist)  
            (t (cons 
                    (cons (car formals) (my-eval (car actuals) alist)) 
                    (my-bind-formals (cdr formals) (cdr actuals) alist))
            )
    )
)  



(defun my-apply-atom (fn args alist)
    (cond 
        ((eq fn 'eq)
        (eq (my-eval (car args) alist) (my-eval (cadr args) alist)))
        ((eq fn 'car)
        (car (my-eval (car args) alist))
        )
        ((eq fn 'cdr)
        (cdr (my-eval (car args) alist))
        )
        ((eq fn 'cons)
        (
            cons 
                (my-eval (car args) alist)   
                (my-eval (cadr args) alist)
        )
        )
        ((eq fn 'quote)
        (car args)
        )
        ((eq fn 'setq) 
        (my-eval-setq 
            (car args) 
            (my-eval (cadr args) alist))
        )
        ((eq fn '+) 
            (+ (my-eval (car args) alist))
        )
        ((eq fn '-) (- (my-eval (car args) alist)))
        ((eq fn '*) (* (my-eval (car args) alist)))
        ((eq fn 'atom) (atom (my-eval (car args) alist)))
        ((eq fn 'null) (null (my-eval (car args) alist)))
        ((eq fn 'print) (print (my-eval (car args) alist))) 
        ((eq fn 'mod) (mod (my-eval (car args) alist) (my-eval (cadr args) alist)))
        ((eq fn 'floor) (floor (my-eval (car args) alist)))
        ((eq fn 'cond) (my-eval-cond args alist))
        ((eq fn 'defun) (my-eval-defun args alist))
        ((eq fn 'eval) (my-eval (my-eval (car args) alist) alist))
        (t (my-apply (cdr (my-assoc fn global-alist)) args alist))  


    )
)

(defun my-eval-setq (var val)

    (setq global-alist (cons (cons var val) global-alist))
    (cdr (my-assoc var global-alist))
)

(defun my-eval-cond (clauses alist)
    (
        cond
            ((null clauses) NIL)
            ((my-eval (car(car clauses)) alist) (my-eval (cadar clauses) alist))
            (t (my-eval-cond (cdr clauses) alist))
    )
)

(defun my-eval-defun (body alist)
    (setq global-alist (cons (cons (car body) (cdr body)) global-alist))
    (car body)
)

(defvar global-alist nil)

(defun my-top ()
    (prog ()
        top
            (print (my-eval (read) global-alist))
            (terpri) 
            (go top) 
    )
)


